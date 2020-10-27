use derive_more::From;

mod io;
pub(self) mod phases;

use io::{CoordinatorClient, ModelStore, PhaseIO, StateMachineIO};
use phases::{Awaiting, NewRound, Phase, SerializableState, Sum, Sum2, Update};

/// A potential transition from one state to another.
pub enum TransitionOutcome<IO> {
    /// A transition is pending. The state machine has not changed
    Pending(StateMachine<IO>),
    /// A transition occured and resulted in this new state machine
    Complete(StateMachine<IO>),
}

#[derive(From, Debug)]
pub enum StateMachine<IO> {
    NewRound(Phase<NewRound, IO>),
    Awaiting(Phase<Awaiting, IO>),
    Sum(Phase<Sum, IO>),
    Update(Phase<Update, IO>),
    Sum2(Phase<Sum2, IO>),
}

impl<IO> StateMachine<IO>
where
    IO: StateMachineIO,
{
    pub async fn transition(self) -> TransitionOutcome<IO> {
        match self {
            StateMachine::NewRound(phase) => phase.step().await,
            StateMachine::Awaiting(phase) => phase.step().await,
            StateMachine::Sum(phase) => phase.step().await,
            StateMachine::Update(phase) => phase.step().await,
            StateMachine::Sum2(phase) => phase.step().await,
        }
    }

    pub fn save(self) -> SerializableState {
        match self {
            StateMachine::NewRound(phase) => phase.state.into(),
            StateMachine::Awaiting(phase) => phase.state.into(),
            StateMachine::Sum(phase) => phase.state.into(),
            StateMachine::Update(phase) => phase.state.into(),
            StateMachine::Sum2(phase) => phase.state.into(),
        }
    }
}

impl<T, U> StateMachine<PhaseIO<T, U>>
where
    T: CoordinatorClient,
    U: ModelStore,
{
    pub fn restore(state: SerializableState, coordinator: T, model_store: U) -> Self {
        let io = PhaseIO::new(coordinator, model_store);
        match state {
            SerializableState::NewRound(state) => {
                Phase::<NewRound, PhaseIO<T, U>>::new(state, io).into()
            }
            SerializableState::Awaiting(state) => {
                Phase::<Awaiting, PhaseIO<T, U>>::new(state, io).into()
            }
            SerializableState::Sum(state) => Phase::<Sum, PhaseIO<T, U>>::new(state, io).into(),
            SerializableState::Sum2(state) => Phase::<Sum2, PhaseIO<T, U>>::new(state, io).into(),
            SerializableState::Update(state) => {
                Phase::<Update, PhaseIO<T, U>>::new(state, io).into()
            }
        }
    }
}
