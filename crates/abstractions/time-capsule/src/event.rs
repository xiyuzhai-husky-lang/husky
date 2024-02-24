use self::{action::IsTimeCapsuleAction, capsule::TimeCapsule};
use super::*;

pub trait IsTimeCapsuleEvent {
    type Buffer: IsTimeCapsuleEventBuffer<Event = Self>;

    type State: IsTimeCapsuleState<Event = Self>;
}

pub trait IsTimeCapsuleEventBuffer: Default {
    type Event: IsTimeCapsuleEvent<Buffer = Self>;

    fn add_action<A>(&mut self, new_action: &A)
    where
        A: IsTimeCapsuleAction<Event = Self::Event>,
    {
        new_action.add_to_event_buffer(self)
    }

    fn finish(self) -> Option<Self::Event>;
}

pub struct TimeCapsuleEventBuilder<'a, S: IsTimeCapsuleState> {
    capsule: &'a mut TimeCapsule<S>,
    event_buffer: <S::Event as IsTimeCapsuleEvent>::Buffer,
}

impl<'a, S: IsTimeCapsuleState> TimeCapsuleEventBuilder<'a, S> {
    pub fn new(capsule: &'a mut TimeCapsule<S>) -> Self {
        Self {
            capsule,
            event_buffer: Default::default(),
        }
    }

    pub fn add_action<A>(&mut self, new_action: A) -> A::Outcome
    where
        A: IsTimeCapsuleAction<Event = S::Event>,
    {
        self.event_buffer.add_action(&new_action);
        self.capsule.state.add_action(new_action)
    }

    pub fn finish(self) -> Option<S::Event> {
        self.event_buffer.finish()
    }
}
