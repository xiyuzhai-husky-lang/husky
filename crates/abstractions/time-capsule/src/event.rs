use self::{action::IsTimeCapsuleAction, capsule::TimeCapsule};
use super::*;

pub trait IsTimeCapsuleEvent {
    type State: IsTimeCapsuleState;

    /// used for event building
    fn null_event() -> Self;

    /// final
    fn add_action<A>(&mut self, new_action: &A)
    where
        A: IsTimeCapsuleAction<Event = Self>,
    {
        new_action.add_to_event(self)
    }
}

pub struct TimeCapsuleEventBuilder<'a, S: IsTimeCapsuleState> {
    capsule: &'a mut TimeCapsule<S>,
    event: S::Event,
}

impl<'a, S: IsTimeCapsuleState> TimeCapsuleEventBuilder<'a, S> {
    pub fn new(capsule: &'a mut TimeCapsule<S>) -> Self {
        Self {
            capsule,
            event: IsTimeCapsuleEvent::null_event(),
        }
    }

    pub fn add_action<A>(&mut self, new_action: A) -> A::Outcome
    where
        A: IsTimeCapsuleAction<Event = S::Event>,
    {
        self.event.add_action(&new_action);
        self.capsule.state.add_action(new_action)
    }

    pub fn finish(self) -> S::Event {
        self.event
    }
}
