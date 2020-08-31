//! Message buffers.
//!
//! See the [message module] documentation since this is a private module anyways.
//!
//! [message module]: ../index.html

use std::convert::{TryFrom, TryInto};

use anyhow::{anyhow, Context};

use crate::{
    crypto::{ByteObject, PublicSigningKey, SecretSigningKey, Signature},
    message::{Chunk, DecodeError, FromBytes, Payload, Sum, Sum2, ToBytes, Update},
};

pub(crate) mod ranges {
    use std::ops::Range;

    use super::*;
    use crate::message::utils::range;

    /// Byte range corresponding to the signature in a message in a
    /// message header
    pub const SIGNATURE: Range<usize> = range(0, Signature::LENGTH);
    /// Byte range corresponding to the participant public key in a
    /// message header
    pub const PARTICIPANT_PK: Range<usize> = range(SIGNATURE.end, PublicSigningKey::LENGTH);
    /// Byte range corresponding to the length field in a message header
    pub const LENGTH: Range<usize> = range(PARTICIPANT_PK.end, 4);
    /// Byte range corresponding to the tag in a message header
    pub const TAG: usize = LENGTH.end;
    /// Byte range reserved for future use
    pub const RESERVED: Range<usize> = range(TAG + 1, 3);
}

/// Length in bytes of a message header
pub const HEADER_LENGTH: usize = ranges::RESERVED.end;

/// A wrapper around a buffer that contains a [`Message`].
///
/// It provides getters and setters to access the different fields of
/// the message safely. A message is made of a header and a payload:
///
/// ```no_rust
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                           signature                           +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                         participant_pk                        +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +                                                               +
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             length                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |      tag      |                    reserved                   |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                    payload (variable length)                  +
/// |                                                               |
/// ```
///
/// - `signature` contains the signature of the entire message
/// - `participant_pk` contains the public key for verifying the
///   signature
/// - `length` is the length in bytes of the _full_ message, _i.e._
///   including the header. This is a 32 bits field so in theory,
///   messages can be as big as 2^32 = 4,294,967,296 bytes.
/// - `tag` indicates the type of message (sum, update, sum2 or
///   multipart message)
///
/// # Examples
/// ## Reading a sum message
///
/// ```rust
/// use std::convert::TryFrom;
/// use xaynet_core::message::{MessageBuffer, Tag};
///
/// let mut bytes = vec![0x11; 64]; // message signature
/// bytes.extend(vec![0x22; 32]); // participant public signing key
/// bytes.extend(&168_u32.to_be_bytes()); // Length field
/// bytes.push(0x01); // tag (sum message)
/// bytes.extend(vec![0x00, 0x00, 0x00]); // reserved
///
/// // Payload: a sum message contains a signature and an ephemeral public key
/// bytes.extend(vec![0xaa; 32]); // signature
/// bytes.extend(vec![0xbb; 32]); // public key
///
/// let buffer = MessageBuffer::new(&bytes).unwrap();
/// assert_eq!(buffer.signature(), vec![0x11; 64].as_slice());
/// assert_eq!(buffer.participant_pk(), vec![0x22; 32].as_slice());
/// assert_eq!(Tag::try_from(buffer.tag()).unwrap(), Tag::Sum);
/// assert_eq!(
///     buffer.payload(),
///     [vec![0xaa; 32], vec![0xbb; 32]].concat().as_slice()
/// );
/// ```
///
/// ## Writing a sum message
///
/// ```rust
/// use std::convert::TryFrom;
/// use xaynet_core::message::{MessageBuffer, Tag};
///
/// let mut expected = vec![0x11; 64]; // message signature
/// expected.extend(vec![0x22; 32]); // participant public signing key
/// expected.extend(&168_u32.to_be_bytes()); // length field
/// expected.push(0x01); // tag (sum message)
/// expected.extend(vec![0x00, 0x00, 0x00]); // reserved
///
/// // Payload: a sum message contains a signature and an ephemeral public key
/// expected.extend(vec![0xaa; 32]); // signature
/// expected.extend(vec![0xbb; 32]); // public key
///
/// let mut bytes = vec![0; expected.len()];
/// let mut buffer = MessageBuffer::new_unchecked(&mut bytes);
/// buffer
///     .signature_mut()
///     .copy_from_slice(vec![0x11; 64].as_slice());
/// buffer
///     .participant_pk_mut()
///     .copy_from_slice(vec![0x22; 32].as_slice());
/// buffer.set_length(168 as u32);
/// buffer.set_tag(Tag::Sum.into());
/// buffer
///     .payload_mut()
///     .copy_from_slice([vec![0xaa; 32], vec![0xbb; 32]].concat().as_slice());
/// assert_eq!(expected, bytes);
/// ```
///
/// [`Message`]: struct.Message.html
pub struct MessageBuffer<T> {
    inner: T,
}

impl<T: AsRef<[u8]>> MessageBuffer<T> {
    /// Performs bound checks for the various message fields on `bytes` and returns a new
    /// [`MessageBuffer`].
    ///
    /// # Errors
    /// Fails if the `bytes` are smaller than a minimal-sized message buffer.
    pub fn new(bytes: T) -> Result<Self, DecodeError> {
        let buffer = Self { inner: bytes };
        buffer
            .check_buffer_length()
            .context("not a valid MessageBuffer")?;
        Ok(buffer)
    }

    /// Returns a [`MessageBuffer`] without performing any bound checks.
    ///
    /// This means accessing the various fields may panic if the data
    /// is invalid.
    pub fn new_unchecked(bytes: T) -> Self {
        Self { inner: bytes }
    }

    /// Performs bound checks to ensure the fields can be accessed
    /// without panicking.
    pub fn check_buffer_length(&self) -> Result<(), DecodeError> {
        let len = self.inner.as_ref().len();
        if len < HEADER_LENGTH {
            return Err(anyhow!(
                "invalid buffer length: {} < {}",
                len,
                HEADER_LENGTH
            ));
        }
        let expected_len = self.length() as usize;
        let actual_len = self.inner.as_ref().len();
        if actual_len < expected_len {
            return Err(anyhow!(
                "invalid message length: length field says {}, but buffer is {} bytes long",
                expected_len,
                actual_len
            ));
        }
        Ok(())
    }

    /// Gets the tag field.
    ///
    /// # Panics
    /// Accessing the field may panic if the buffer has not been checked before.
    pub fn tag(&self) -> u8 {
        self.inner.as_ref()[ranges::TAG]
    }

    /// Gets the length field
    ///
    /// # Panics
    /// Accessing the field may panic if the buffer has not been checked before.
    pub fn length(&self) -> u32 {
        // Unwrapping is OK, as the slice is guaranteed to be 4 bytes
        // long
        u32::from_be_bytes(self.inner.as_ref()[ranges::LENGTH].try_into().unwrap())
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> MessageBuffer<&'a T> {
    /// Gets the message signature field
    ///
    /// # Panics
    /// Accessing the field may panic if the buffer has not been checked before.
    pub fn signature(&self) -> &'a [u8] {
        &self.inner.as_ref()[ranges::SIGNATURE]
    }

    /// Gets the participant public key field.
    ///
    /// # Panics
    /// Accessing the field may panic if the buffer has not been checked before.
    pub fn participant_pk(&self) -> &'a [u8] {
        &self.inner.as_ref()[ranges::PARTICIPANT_PK]
    }

    /// Gets the rest of the message.
    ///
    /// # Panics
    /// Accessing the field may panic if the buffer has not been checked before.
    pub fn payload(&self) -> &'a [u8] {
        &self.inner.as_ref()[HEADER_LENGTH..]
    }

    /// Parse the signature and public signing key, and check the
    /// message signature.
    pub fn check_signature(&self) -> Result<(), DecodeError> {
        let signature =
            Signature::from_bytes(&self.signature()).context("cannot parse the signature field")?;
        let participant_pk = PublicSigningKey::from_bytes(&self.participant_pk())
            .context("cannot part the public key field")?;

        if participant_pk.verify_detached(&signature, self.signed_data()) {
            Ok(())
        } else {
            Err(anyhow!("invalid message signature"))
        }
    }

    /// Return the portion of the message used to compute the
    /// signature, ie the entire message except the signature field
    /// itself.
    ///
    /// # Panics
    /// Accessing the field may panic if the buffer has not been checked before.
    pub fn signed_data(&self) -> &'a [u8] {
        let signed_data_range = ranges::SIGNATURE.end..self.length() as usize;
        &self.inner.as_ref()[signed_data_range]
    }
}

impl<T: AsMut<[u8]> + AsRef<[u8]>> MessageBuffer<T> {
    /// Sets the tag field.
    ///
    /// # Panics
    /// Accessing the field may panic if the buffer has not been checked before.
    pub fn set_tag(&mut self, value: u8) {
        self.inner.as_mut()[ranges::TAG] = value;
    }

    /// Sets the length field.
    ///
    /// # Panics
    /// Accessing the field may panic if the buffer has not been checked before.
    pub fn set_length(&mut self, value: u32) {
        let bytes = value.to_be_bytes();
        self.inner.as_mut()[ranges::LENGTH].copy_from_slice(&bytes[..]);
    }

    /// Gets a mutable reference to the message signature field.
    ///
    /// # Panics
    /// Accessing the field may panic if the buffer has not been checked before.
    pub fn signature_mut(&mut self) -> &mut [u8] {
        &mut self.inner.as_mut()[ranges::SIGNATURE]
    }

    /// Gets a mutable reference to the participant public key field.
    ///
    /// # Panics
    /// Accessing the field may panic if the buffer has not been checked before.
    pub fn participant_pk_mut(&mut self) -> &mut [u8] {
        &mut self.inner.as_mut()[ranges::PARTICIPANT_PK]
    }

    /// Gets a mutable reference to the rest of the message.
    ///
    /// # Panics
    /// Accessing the field may panic if the buffer has not been checked before.
    pub fn payload_mut(&mut self) -> &mut [u8] {
        &mut self.inner.as_mut()[HEADER_LENGTH..]
    }

    /// Gets a mutable reference to the portion of the message used to
    /// compute the signature, ie the entire message except the
    /// signature field itself.
    ///
    /// # Panics
    /// Accessing the field may panic if the buffer has not been checked before.
    pub fn signed_data_mut(&mut self) -> &mut [u8] {
        let signed_data_range = ranges::SIGNATURE.end..self.length() as usize;
        &mut self.inner.as_mut()[signed_data_range]
    }
}

#[derive(Copy, Debug, Clone, Eq, PartialEq)]
/// A tag that indicates the type of the [`Message`].
pub enum Tag {
    /// A tag for [`Sum`] messages
    Sum,
    /// A tag for [`Update`] messages
    Update,
    /// A tag for [`Sum2`] messages
    Sum2,
    /// A tag for [`Chunk`] messages
    Chunk,
}

impl TryFrom<u8> for Tag {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Tag::Sum,
            2 => Tag::Update,
            3 => Tag::Sum2,
            4 => Tag::Chunk,
            _ => return Err(anyhow!("invalid tag {}", value)),
        })
    }
}

impl Into<u8> for Tag {
    fn into(self) -> u8 {
        match self {
            Tag::Sum => 1,
            Tag::Update => 2,
            Tag::Sum2 => 3,
            Tag::Chunk => 4,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// A header common to all [`Message`]s.
pub struct Message {
    /// Message signature. This can be `None` if it hasn't been
    /// computed yet.
    pub signature: Option<Signature>,
    /// The participant public key, used to verify the message
    /// signature.
    pub participant_pk: PublicSigningKey,
    /// Message payload
    pub payload: Payload,
}

impl Message {
    /// Parse the given message **without** verifying the
    /// signature. If you need to check the signature, call
    /// [`MessageBuffer.verify_signature`] before parsing the message.
    pub fn from_bytes<T: AsRef<[u8]>>(buffer: &T) -> Result<Self, DecodeError> {
        let reader = MessageBuffer::new(buffer.as_ref())?;
        let participant_pk = PublicSigningKey::from_bytes(&reader.participant_pk())
            .context("failed to parse public key")?;
        let signature =
            Signature::from_bytes(&reader.signature()).context("failed to parse signature")?;

        let payload = match reader.tag().try_into()? {
            Tag::Sum => Sum::from_bytes(&reader.payload()).map(Into::into),
            Tag::Update => Update::from_bytes(&reader.payload()).map(Into::into),
            Tag::Sum2 => Sum2::from_bytes(&reader.payload()).map(Into::into),
            Tag::Chunk => Chunk::from_bytes(&reader.payload()).map(Into::into),
        }
        .context("failed to parse message payload")?;

        Ok(Self {
            participant_pk,
            signature: Some(signature),
            payload,
        })
    }

    /// Serialize this message. If the `signature` attribute is
    /// `Some`, the signature will be directly inserted in the message
    /// header. Otherwise it will be computed.
    ///
    /// # Panic
    ///
    /// This method panics if the given buffer is too small for the
    /// message to fit.
    pub fn to_bytes<T: AsMut<[u8]> + AsRef<[u8]> + ?Sized>(
        &self,
        buffer: &mut T,
        sk: &SecretSigningKey,
    ) {
        let mut writer = MessageBuffer::new(buffer.as_mut()).unwrap();

        self.participant_pk
            .to_bytes(&mut writer.participant_pk_mut());

        let tag = match self.payload {
            Payload::Sum(_) => Tag::Sum,
            Payload::Update(_) => Tag::Update,
            Payload::Sum2(_) => Tag::Sum2,
            Payload::Chunk(_) => Tag::Chunk,
        };
        writer.set_tag(tag.into());
        self.payload.to_bytes(&mut writer.payload_mut());
        writer.set_length(self.buffer_length() as u32);
        // insert the signature last. If the message contains one, use
        // it. Otherwise compute it.
        let signature = match self.signature {
            Some(signature) => signature,
            None => sk.sign_detached(&writer.signed_data_mut()),
        };
        signature.to_bytes(&mut writer.signature_mut());
    }

    pub fn buffer_length(&self) -> usize {
        self.payload.buffer_length() + HEADER_LENGTH
    }
}

#[cfg(test)]
pub(in crate::message) mod tests {
    use std::convert::TryFrom;

    use super::*;
    use crate::{
        crypto::{ByteObject, PublicSigningKey, Signature},
        message::{payload::sum, Message, Tag},
    };

    fn signature() -> (Vec<u8>, Signature) {
        let bytes = vec![0xaa; 64];
        let signature = Signature::from_slice(bytes.as_slice()).unwrap();
        (bytes, signature)
    }

    fn participant_pk() -> (Vec<u8>, PublicSigningKey) {
        let bytes = vec![0xbb; 32];
        let pk = PublicSigningKey::from_slice(&bytes).unwrap();
        (bytes, pk)
    }

    pub(crate) fn message() -> (Vec<u8>, Message) {
        let message = Message {
            signature: Some(signature().1),
            participant_pk: participant_pk().1,
            payload: sum::tests::sum().into(),
        };

        let mut buf = signature().0;
        buf.extend(participant_pk().0);
        let length = sum::tests::sum_bytes().len() + HEADER_LENGTH;
        buf.extend(&(length as u32).to_be_bytes());
        buf.push(Tag::Sum.into());
        buf.extend(vec![0, 0, 0]);
        buf.extend(sum::tests::sum_bytes());

        (buf, message)
    }

    #[test]
    fn buffer_read() {
        let bytes = message().0;
        let buffer = MessageBuffer::new(&bytes).unwrap();
        assert_eq!(Tag::try_from(buffer.tag()).unwrap(), Tag::Sum);
        assert_eq!(buffer.signature(), signature().0.as_slice());
        assert_eq!(buffer.participant_pk(), participant_pk().0.as_slice());
        assert_eq!(buffer.length() as usize, bytes.len());
        assert_eq!(buffer.payload(), sum::tests::sum_bytes().as_slice());
    }

    #[test]
    fn buffer_write() {
        let expected = message().0;
        let mut bytes = vec![0; expected.len()];
        let mut buffer = MessageBuffer::new_unchecked(&mut bytes);

        buffer
            .signature_mut()
            .copy_from_slice(signature().0.as_slice());
        buffer
            .participant_pk_mut()
            .copy_from_slice(participant_pk().0.as_slice());
        buffer.set_tag(Tag::Sum.into());
        buffer.set_length(expected.len() as u32);
        buffer
            .payload_mut()
            .copy_from_slice(sum::tests::sum_bytes().as_slice());
        assert_eq!(bytes, expected);
    }
}
