use crate::{action::IsTimeCapsuleAction, event::IsTimeCapsuleEvent};

pub trait IsTimeCapsuleState {
    type Event: IsTimeCapsuleEvent<State = Self>;

    fn redo(&mut self, event: &Self::Event);
    fn undo(&mut self, event: &Self::Event);
    /// final
    fn add_action<A>(&mut self, new_action: A) -> A::Outcome
    where
        A: IsTimeCapsuleAction<Event = Self::Event>,
    {
        new_action.exec(self)
    }
}
