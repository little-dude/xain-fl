// Important the macro_use modules must be declared first for the
// macro to be used in the other modules
#[macro_use]
mod phase;
mod io;
mod phases;
mod state_machine;

// It is useful to re-export everything within this module because
// there are lot of interdependencies between all the sub-modules
pub(self) use self::{
    io::{boxed_io, IO},
    phase::{Phase, Progress, SerializableState, SharedState, State, Step},
    phases::{Awaiting, NewRound, Sum, Sum2, Update},
};

pub use state_machine::{StateMachine, TransitionOutcome};
