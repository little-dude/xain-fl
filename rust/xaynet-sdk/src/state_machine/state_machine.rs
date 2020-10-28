use derive_more::From;

use crate::{settings::Settings, ModelStore, Notify, XaynetClient};

use super::{
    boxed_io,
    Awaiting,
    NewRound,
    Phase,
    SerializableState,
    SharedState,
    State,
    Sum,
    Sum2,
    Update,
};

/// A potential transition from one state to another.
pub enum TransitionOutcome {
    /// A transition is pending. The state machine has not changed
    Pending(StateMachine),
    /// A transition occured and resulted in this new state machine
    Complete(StateMachine),
}

#[derive(From)]
pub enum StateMachine {
    NewRound(Phase<NewRound>),
    Awaiting(Phase<Awaiting>),
    Sum(Phase<Sum>),
    Update(Phase<Update>),
    Sum2(Phase<Sum2>),
}

impl StateMachine {
    pub async fn transition(self) -> TransitionOutcome {
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

impl StateMachine {
    pub fn new<X, M, N>(settings: Settings, xaynet_client: X, model_store: M, notifier: N) -> Self
    where
        X: XaynetClient + Send + 'static,
        M: ModelStore + Send + 'static,
        N: Notify + Send + 'static,
    {
        let io = boxed_io(xaynet_client, model_store, notifier);
        let state = State::new(SharedState::new(settings), Awaiting);
        Phase::<_>::new(state, io).into()
    }

    pub fn restore<X, M, N>(
        state: SerializableState,
        xaynet_client: X,
        model_store: M,
        notifier: N,
    ) -> Self
    where
        X: XaynetClient + Send + 'static,
        M: ModelStore + Send + 'static,
        N: Notify + Send + 'static,
    {
        let io = boxed_io(xaynet_client, model_store, notifier);
        match state {
            SerializableState::NewRound(state) => Phase::<_>::new(state, io).into(),
            SerializableState::Awaiting(state) => Phase::<_>::new(state, io).into(),
            SerializableState::Sum(state) => Phase::<_>::new(state, io).into(),
            SerializableState::Sum2(state) => Phase::<_>::new(state, io).into(),
            SerializableState::Update(state) => Phase::<_>::new(state, io).into(),
        }
    }
}
