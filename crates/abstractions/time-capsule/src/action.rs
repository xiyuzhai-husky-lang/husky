use crate::event::IsTimeCapsuleEvent;

pub trait IsTimeCapsuleAction {
    type Event: IsTimeCapsuleEvent;

    type Outcome;

    fn add_to_event(&self, event: &mut Self::Event);

    fn exec(self, state: &mut <Self::Event as IsTimeCapsuleEvent>::State) -> Self::Outcome;
}
