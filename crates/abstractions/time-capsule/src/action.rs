use crate::event::IsTimeCapsuleEvent;

pub trait IsTimeCapsuleAction {
    type Event: IsTimeCapsuleEvent;

    type Outcome;

    fn add_to_event_buffer(&self, event: &mut <Self::Event as IsTimeCapsuleEvent>::Buffer);

    fn exec(self, state: &mut <Self::Event as IsTimeCapsuleEvent>::State) -> Self::Outcome;
}
