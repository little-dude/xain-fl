use derive_more::From;

mod io;
pub(self) mod phases;

use phases::{Awaiting, NewRound, Phase, PhaseState, SerializableState, Sum, Sum2, Update};

pub use phases::TransitionOutcome;

use io::{CoordinatorClient, ModelStore, PhaseIO, StateMachineIO};

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
            StateMachine::NewRound(phase) => phase.step().await.into(),
            StateMachine::Awaiting(phase) => phase.step().await.into(),
            StateMachine::Sum(phase) => phase.step().await.into(),
            StateMachine::Update(phase) => phase.step().await.into(),
            StateMachine::Sum2(phase) => phase.step().await.into(),
        }
    }

    pub fn save(self) -> PhaseState {
        match self {
            StateMachine::NewRound(phase) => phase.phase_state.into(),
            StateMachine::Awaiting(phase) => phase.phase_state.into(),
            StateMachine::Sum(phase) => phase.phase_state.into(),
            StateMachine::Update(phase) => phase.phase_state.into(),
            StateMachine::Sum2(phase) => phase.phase_state.into(),
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
        let SerializableState { shared, phase } = state;
        match phase {
            PhaseState::NewRound(phase_state) => {
                Phase::<NewRound, PhaseIO<T, U>>::restore(shared, phase_state, io).into()
            }
            PhaseState::Awaiting(phase_state) => {
                Phase::<Awaiting, PhaseIO<T, U>>::restore(shared, phase_state, io).into()
            }
            PhaseState::Sum(phase_state) => {
                Phase::<Sum, PhaseIO<T, U>>::restore(shared, phase_state, io).into()
            }
            PhaseState::Sum2(phase_state) => {
                Phase::<Sum2, PhaseIO<T, U>>::restore(shared, phase_state, io).into()
            }
            PhaseState::Update(phase_state) => {
                Phase::<Update, PhaseIO<T, U>>::restore(shared, phase_state, io).into()
            }
        }
    }
}
