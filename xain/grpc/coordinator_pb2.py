# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: xain/grpc/coordinator.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from numproto.protobuf import ndarray_pb2 as numproto_dot_protobuf_dot_ndarray__pb2


DESCRIPTOR = _descriptor.FileDescriptor(
  name='xain/grpc/coordinator.proto',
  package='xain.protobuf.coordinator',
  syntax='proto3',
  serialized_options=None,
  serialized_pb=_b('\n\x1bxain/grpc/coordinator.proto\x12\x19xain.protobuf.coordinator\x1a\x1fnumproto/protobuf/ndarray.proto\"\x13\n\x11RendezvousRequest\"R\n\x0fRendezvousReply\x12?\n\x08response\x18\x01 \x01(\x0e\x32-.xain.protobuf.coordinator.RendezvousResponse\"R\n\x10HeartbeatRequest\x12/\n\x05state\x18\x01 \x01(\x0e\x32 .xain.protobuf.coordinator.State\x12\r\n\x05round\x18\x02 \x01(\x05\"P\n\x0eHeartbeatReply\x12/\n\x05state\x18\x01 \x01(\x0e\x32 .xain.protobuf.coordinator.State\x12\r\n\x05round\x18\x02 \x01(\x05\"\x16\n\x14StartTrainingRequest\"c\n\x12StartTrainingReply\x12)\n\x05theta\x18\x01 \x03(\x0b\x32\x1a.numproto.protobuf.NDArray\x12\x0e\n\x06\x65pochs\x18\x02 \x01(\x05\x12\x12\n\nepoch_base\x18\x03 \x01(\x05\"\x8a\x04\n\x12\x45ndTrainingRequest\x12O\n\x0ctheta_update\x18\x01 \x01(\x0b\x32\x39.xain.protobuf.coordinator.EndTrainingRequest.ThetaUpdate\x12K\n\x07history\x18\x02 \x03(\x0b\x32:.xain.protobuf.coordinator.EndTrainingRequest.HistoryEntry\x12\x46\n\x07metrics\x18\x03 \x01(\x0b\x32\x35.xain.protobuf.coordinator.EndTrainingRequest.Metrics\x1aj\n\x0cHistoryEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12I\n\x05value\x18\x02 \x01(\x0b\x32:.xain.protobuf.coordinator.EndTrainingRequest.HistoryValue:\x02\x38\x01\x1aT\n\x0bThetaUpdate\x12/\n\x0btheta_prime\x18\x01 \x03(\x0b\x32\x1a.numproto.protobuf.NDArray\x12\x14\n\x0cnum_examples\x18\x02 \x01(\x05\x1a\x1e\n\x0cHistoryValue\x12\x0e\n\x06values\x18\x01 \x03(\x02\x1a,\n\x07Metrics\x12\x0b\n\x03\x63id\x18\x01 \x01(\x05\x12\x14\n\x0cvol_by_class\x18\x02 \x03(\x05\"\x12\n\x10\x45ndTrainingReply*+\n\x12RendezvousResponse\x12\n\n\x06\x41\x43\x43\x45PT\x10\x00\x12\t\n\x05LATER\x10\x01*F\n\x05State\x12\x0b\n\x07STANDBY\x10\x00\x12\t\n\x05ROUND\x10\x01\x12\x0c\n\x08\x46INISHED\x10\x02\x12\t\n\x05READY\x10\x03\x12\x0c\n\x08TRAINING\x10\x04\x32\xbe\x03\n\x0b\x43oordinator\x12h\n\nRendezvous\x12,.xain.protobuf.coordinator.RendezvousRequest\x1a*.xain.protobuf.coordinator.RendezvousReply\"\x00\x12\x65\n\tHeartbeat\x12+.xain.protobuf.coordinator.HeartbeatRequest\x1a).xain.protobuf.coordinator.HeartbeatReply\"\x00\x12q\n\rStartTraining\x12/.xain.protobuf.coordinator.StartTrainingRequest\x1a-.xain.protobuf.coordinator.StartTrainingReply\"\x00\x12k\n\x0b\x45ndTraining\x12-.xain.protobuf.coordinator.EndTrainingRequest\x1a+.xain.protobuf.coordinator.EndTrainingReply\"\x00\x62\x06proto3')
  ,
  dependencies=[numproto_dot_protobuf_dot_ndarray__pb2.DESCRIPTOR,])

_RENDEZVOUSRESPONSE = _descriptor.EnumDescriptor(
  name='RendezvousResponse',
  full_name='xain.protobuf.coordinator.RendezvousResponse',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='ACCEPT', index=0, number=0,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='LATER', index=1, number=1,
      serialized_options=None,
      type=None),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=1032,
  serialized_end=1075,
)
_sym_db.RegisterEnumDescriptor(_RENDEZVOUSRESPONSE)

RendezvousResponse = enum_type_wrapper.EnumTypeWrapper(_RENDEZVOUSRESPONSE)
_STATE = _descriptor.EnumDescriptor(
  name='State',
  full_name='xain.protobuf.coordinator.State',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='STANDBY', index=0, number=0,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='ROUND', index=1, number=1,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='FINISHED', index=2, number=2,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='READY', index=3, number=3,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='TRAINING', index=4, number=4,
      serialized_options=None,
      type=None),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=1077,
  serialized_end=1147,
)
_sym_db.RegisterEnumDescriptor(_STATE)

State = enum_type_wrapper.EnumTypeWrapper(_STATE)
ACCEPT = 0
LATER = 1
STANDBY = 0
ROUND = 1
FINISHED = 2
READY = 3
TRAINING = 4



_RENDEZVOUSREQUEST = _descriptor.Descriptor(
  name='RendezvousRequest',
  full_name='xain.protobuf.coordinator.RendezvousRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=91,
  serialized_end=110,
)


_RENDEZVOUSREPLY = _descriptor.Descriptor(
  name='RendezvousReply',
  full_name='xain.protobuf.coordinator.RendezvousReply',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='response', full_name='xain.protobuf.coordinator.RendezvousReply.response', index=0,
      number=1, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=112,
  serialized_end=194,
)


_HEARTBEATREQUEST = _descriptor.Descriptor(
  name='HeartbeatRequest',
  full_name='xain.protobuf.coordinator.HeartbeatRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='state', full_name='xain.protobuf.coordinator.HeartbeatRequest.state', index=0,
      number=1, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='round', full_name='xain.protobuf.coordinator.HeartbeatRequest.round', index=1,
      number=2, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=196,
  serialized_end=278,
)


_HEARTBEATREPLY = _descriptor.Descriptor(
  name='HeartbeatReply',
  full_name='xain.protobuf.coordinator.HeartbeatReply',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='state', full_name='xain.protobuf.coordinator.HeartbeatReply.state', index=0,
      number=1, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='round', full_name='xain.protobuf.coordinator.HeartbeatReply.round', index=1,
      number=2, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=280,
  serialized_end=360,
)


_STARTTRAININGREQUEST = _descriptor.Descriptor(
  name='StartTrainingRequest',
  full_name='xain.protobuf.coordinator.StartTrainingRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=362,
  serialized_end=384,
)


_STARTTRAININGREPLY = _descriptor.Descriptor(
  name='StartTrainingReply',
  full_name='xain.protobuf.coordinator.StartTrainingReply',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='theta', full_name='xain.protobuf.coordinator.StartTrainingReply.theta', index=0,
      number=1, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='epochs', full_name='xain.protobuf.coordinator.StartTrainingReply.epochs', index=1,
      number=2, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='epoch_base', full_name='xain.protobuf.coordinator.StartTrainingReply.epoch_base', index=2,
      number=3, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=386,
  serialized_end=485,
)


_ENDTRAININGREQUEST_HISTORYENTRY = _descriptor.Descriptor(
  name='HistoryEntry',
  full_name='xain.protobuf.coordinator.EndTrainingRequest.HistoryEntry',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='key', full_name='xain.protobuf.coordinator.EndTrainingRequest.HistoryEntry.key', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='value', full_name='xain.protobuf.coordinator.EndTrainingRequest.HistoryEntry.value', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=_b('8\001'),
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=740,
  serialized_end=846,
)

_ENDTRAININGREQUEST_THETAUPDATE = _descriptor.Descriptor(
  name='ThetaUpdate',
  full_name='xain.protobuf.coordinator.EndTrainingRequest.ThetaUpdate',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='theta_prime', full_name='xain.protobuf.coordinator.EndTrainingRequest.ThetaUpdate.theta_prime', index=0,
      number=1, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='num_examples', full_name='xain.protobuf.coordinator.EndTrainingRequest.ThetaUpdate.num_examples', index=1,
      number=2, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=848,
  serialized_end=932,
)

_ENDTRAININGREQUEST_HISTORYVALUE = _descriptor.Descriptor(
  name='HistoryValue',
  full_name='xain.protobuf.coordinator.EndTrainingRequest.HistoryValue',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='values', full_name='xain.protobuf.coordinator.EndTrainingRequest.HistoryValue.values', index=0,
      number=1, type=2, cpp_type=6, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=934,
  serialized_end=964,
)

_ENDTRAININGREQUEST_METRICS = _descriptor.Descriptor(
  name='Metrics',
  full_name='xain.protobuf.coordinator.EndTrainingRequest.Metrics',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='cid', full_name='xain.protobuf.coordinator.EndTrainingRequest.Metrics.cid', index=0,
      number=1, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='vol_by_class', full_name='xain.protobuf.coordinator.EndTrainingRequest.Metrics.vol_by_class', index=1,
      number=2, type=5, cpp_type=1, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=966,
  serialized_end=1010,
)

_ENDTRAININGREQUEST = _descriptor.Descriptor(
  name='EndTrainingRequest',
  full_name='xain.protobuf.coordinator.EndTrainingRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='theta_update', full_name='xain.protobuf.coordinator.EndTrainingRequest.theta_update', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='history', full_name='xain.protobuf.coordinator.EndTrainingRequest.history', index=1,
      number=2, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='metrics', full_name='xain.protobuf.coordinator.EndTrainingRequest.metrics', index=2,
      number=3, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[_ENDTRAININGREQUEST_HISTORYENTRY, _ENDTRAININGREQUEST_THETAUPDATE, _ENDTRAININGREQUEST_HISTORYVALUE, _ENDTRAININGREQUEST_METRICS, ],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=488,
  serialized_end=1010,
)


_ENDTRAININGREPLY = _descriptor.Descriptor(
  name='EndTrainingReply',
  full_name='xain.protobuf.coordinator.EndTrainingReply',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=1012,
  serialized_end=1030,
)

_RENDEZVOUSREPLY.fields_by_name['response'].enum_type = _RENDEZVOUSRESPONSE
_HEARTBEATREQUEST.fields_by_name['state'].enum_type = _STATE
_HEARTBEATREPLY.fields_by_name['state'].enum_type = _STATE
_STARTTRAININGREPLY.fields_by_name['theta'].message_type = numproto_dot_protobuf_dot_ndarray__pb2._NDARRAY
_ENDTRAININGREQUEST_HISTORYENTRY.fields_by_name['value'].message_type = _ENDTRAININGREQUEST_HISTORYVALUE
_ENDTRAININGREQUEST_HISTORYENTRY.containing_type = _ENDTRAININGREQUEST
_ENDTRAININGREQUEST_THETAUPDATE.fields_by_name['theta_prime'].message_type = numproto_dot_protobuf_dot_ndarray__pb2._NDARRAY
_ENDTRAININGREQUEST_THETAUPDATE.containing_type = _ENDTRAININGREQUEST
_ENDTRAININGREQUEST_HISTORYVALUE.containing_type = _ENDTRAININGREQUEST
_ENDTRAININGREQUEST_METRICS.containing_type = _ENDTRAININGREQUEST
_ENDTRAININGREQUEST.fields_by_name['theta_update'].message_type = _ENDTRAININGREQUEST_THETAUPDATE
_ENDTRAININGREQUEST.fields_by_name['history'].message_type = _ENDTRAININGREQUEST_HISTORYENTRY
_ENDTRAININGREQUEST.fields_by_name['metrics'].message_type = _ENDTRAININGREQUEST_METRICS
DESCRIPTOR.message_types_by_name['RendezvousRequest'] = _RENDEZVOUSREQUEST
DESCRIPTOR.message_types_by_name['RendezvousReply'] = _RENDEZVOUSREPLY
DESCRIPTOR.message_types_by_name['HeartbeatRequest'] = _HEARTBEATREQUEST
DESCRIPTOR.message_types_by_name['HeartbeatReply'] = _HEARTBEATREPLY
DESCRIPTOR.message_types_by_name['StartTrainingRequest'] = _STARTTRAININGREQUEST
DESCRIPTOR.message_types_by_name['StartTrainingReply'] = _STARTTRAININGREPLY
DESCRIPTOR.message_types_by_name['EndTrainingRequest'] = _ENDTRAININGREQUEST
DESCRIPTOR.message_types_by_name['EndTrainingReply'] = _ENDTRAININGREPLY
DESCRIPTOR.enum_types_by_name['RendezvousResponse'] = _RENDEZVOUSRESPONSE
DESCRIPTOR.enum_types_by_name['State'] = _STATE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

RendezvousRequest = _reflection.GeneratedProtocolMessageType('RendezvousRequest', (_message.Message,), {
  'DESCRIPTOR' : _RENDEZVOUSREQUEST,
  '__module__' : 'xain.grpc.coordinator_pb2'
  # @@protoc_insertion_point(class_scope:xain.protobuf.coordinator.RendezvousRequest)
  })
_sym_db.RegisterMessage(RendezvousRequest)

RendezvousReply = _reflection.GeneratedProtocolMessageType('RendezvousReply', (_message.Message,), {
  'DESCRIPTOR' : _RENDEZVOUSREPLY,
  '__module__' : 'xain.grpc.coordinator_pb2'
  # @@protoc_insertion_point(class_scope:xain.protobuf.coordinator.RendezvousReply)
  })
_sym_db.RegisterMessage(RendezvousReply)

HeartbeatRequest = _reflection.GeneratedProtocolMessageType('HeartbeatRequest', (_message.Message,), {
  'DESCRIPTOR' : _HEARTBEATREQUEST,
  '__module__' : 'xain.grpc.coordinator_pb2'
  # @@protoc_insertion_point(class_scope:xain.protobuf.coordinator.HeartbeatRequest)
  })
_sym_db.RegisterMessage(HeartbeatRequest)

HeartbeatReply = _reflection.GeneratedProtocolMessageType('HeartbeatReply', (_message.Message,), {
  'DESCRIPTOR' : _HEARTBEATREPLY,
  '__module__' : 'xain.grpc.coordinator_pb2'
  # @@protoc_insertion_point(class_scope:xain.protobuf.coordinator.HeartbeatReply)
  })
_sym_db.RegisterMessage(HeartbeatReply)

StartTrainingRequest = _reflection.GeneratedProtocolMessageType('StartTrainingRequest', (_message.Message,), {
  'DESCRIPTOR' : _STARTTRAININGREQUEST,
  '__module__' : 'xain.grpc.coordinator_pb2'
  # @@protoc_insertion_point(class_scope:xain.protobuf.coordinator.StartTrainingRequest)
  })
_sym_db.RegisterMessage(StartTrainingRequest)

StartTrainingReply = _reflection.GeneratedProtocolMessageType('StartTrainingReply', (_message.Message,), {
  'DESCRIPTOR' : _STARTTRAININGREPLY,
  '__module__' : 'xain.grpc.coordinator_pb2'
  # @@protoc_insertion_point(class_scope:xain.protobuf.coordinator.StartTrainingReply)
  })
_sym_db.RegisterMessage(StartTrainingReply)

EndTrainingRequest = _reflection.GeneratedProtocolMessageType('EndTrainingRequest', (_message.Message,), {

  'HistoryEntry' : _reflection.GeneratedProtocolMessageType('HistoryEntry', (_message.Message,), {
    'DESCRIPTOR' : _ENDTRAININGREQUEST_HISTORYENTRY,
    '__module__' : 'xain.grpc.coordinator_pb2'
    # @@protoc_insertion_point(class_scope:xain.protobuf.coordinator.EndTrainingRequest.HistoryEntry)
    })
  ,

  'ThetaUpdate' : _reflection.GeneratedProtocolMessageType('ThetaUpdate', (_message.Message,), {
    'DESCRIPTOR' : _ENDTRAININGREQUEST_THETAUPDATE,
    '__module__' : 'xain.grpc.coordinator_pb2'
    # @@protoc_insertion_point(class_scope:xain.protobuf.coordinator.EndTrainingRequest.ThetaUpdate)
    })
  ,

  'HistoryValue' : _reflection.GeneratedProtocolMessageType('HistoryValue', (_message.Message,), {
    'DESCRIPTOR' : _ENDTRAININGREQUEST_HISTORYVALUE,
    '__module__' : 'xain.grpc.coordinator_pb2'
    # @@protoc_insertion_point(class_scope:xain.protobuf.coordinator.EndTrainingRequest.HistoryValue)
    })
  ,

  'Metrics' : _reflection.GeneratedProtocolMessageType('Metrics', (_message.Message,), {
    'DESCRIPTOR' : _ENDTRAININGREQUEST_METRICS,
    '__module__' : 'xain.grpc.coordinator_pb2'
    # @@protoc_insertion_point(class_scope:xain.protobuf.coordinator.EndTrainingRequest.Metrics)
    })
  ,
  'DESCRIPTOR' : _ENDTRAININGREQUEST,
  '__module__' : 'xain.grpc.coordinator_pb2'
  # @@protoc_insertion_point(class_scope:xain.protobuf.coordinator.EndTrainingRequest)
  })
_sym_db.RegisterMessage(EndTrainingRequest)
_sym_db.RegisterMessage(EndTrainingRequest.HistoryEntry)
_sym_db.RegisterMessage(EndTrainingRequest.ThetaUpdate)
_sym_db.RegisterMessage(EndTrainingRequest.HistoryValue)
_sym_db.RegisterMessage(EndTrainingRequest.Metrics)

EndTrainingReply = _reflection.GeneratedProtocolMessageType('EndTrainingReply', (_message.Message,), {
  'DESCRIPTOR' : _ENDTRAININGREPLY,
  '__module__' : 'xain.grpc.coordinator_pb2'
  # @@protoc_insertion_point(class_scope:xain.protobuf.coordinator.EndTrainingReply)
  })
_sym_db.RegisterMessage(EndTrainingReply)


_ENDTRAININGREQUEST_HISTORYENTRY._options = None

_COORDINATOR = _descriptor.ServiceDescriptor(
  name='Coordinator',
  full_name='xain.protobuf.coordinator.Coordinator',
  file=DESCRIPTOR,
  index=0,
  serialized_options=None,
  serialized_start=1150,
  serialized_end=1596,
  methods=[
  _descriptor.MethodDescriptor(
    name='Rendezvous',
    full_name='xain.protobuf.coordinator.Coordinator.Rendezvous',
    index=0,
    containing_service=None,
    input_type=_RENDEZVOUSREQUEST,
    output_type=_RENDEZVOUSREPLY,
    serialized_options=None,
  ),
  _descriptor.MethodDescriptor(
    name='Heartbeat',
    full_name='xain.protobuf.coordinator.Coordinator.Heartbeat',
    index=1,
    containing_service=None,
    input_type=_HEARTBEATREQUEST,
    output_type=_HEARTBEATREPLY,
    serialized_options=None,
  ),
  _descriptor.MethodDescriptor(
    name='StartTraining',
    full_name='xain.protobuf.coordinator.Coordinator.StartTraining',
    index=2,
    containing_service=None,
    input_type=_STARTTRAININGREQUEST,
    output_type=_STARTTRAININGREPLY,
    serialized_options=None,
  ),
  _descriptor.MethodDescriptor(
    name='EndTraining',
    full_name='xain.protobuf.coordinator.Coordinator.EndTraining',
    index=3,
    containing_service=None,
    input_type=_ENDTRAININGREQUEST,
    output_type=_ENDTRAININGREPLY,
    serialized_options=None,
  ),
])
_sym_db.RegisterServiceDescriptor(_COORDINATOR)

DESCRIPTOR.services_by_name['Coordinator'] = _COORDINATOR

# @@protoc_insertion_point(module_scope)
