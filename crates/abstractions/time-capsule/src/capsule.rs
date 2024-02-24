use super::*;
use crate::event::TimeCapsuleEventBuilder;

#[derive(Debug)]
pub struct TimeCapsule<S: IsTimeCapsuleState> {
    pub(crate) state: S,
    events: Vec<S::Event>,
    num_of_active_events: usize,
}

impl<S: IsTimeCapsuleState> std::ops::Deref for TimeCapsule<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<S: IsTimeCapsuleState> Default for TimeCapsule<S>
where
    S: Default,
{
    fn default() -> Self {
        Self {
            state: Default::default(),
            events: Default::default(),
            num_of_active_events: 0,
        }
    }
}

impl<S: IsTimeCapsuleState> TimeCapsule<S> {
    pub fn undo(&mut self) {
        if self.num_of_active_events > 0 {
            let prev_event = self.num_of_active_events - 1;
            self.state.undo(&self.events[prev_event]);
            self.num_of_active_events -= 1;
        }
    }

    pub fn redo(&mut self) {
        if self.num_of_active_events < self.events.len() {
            let next_event = self.num_of_active_events;
            self.state.redo(&self.events[next_event]);
            self.num_of_active_events += 1;
        }
    }

    pub fn add_event<R>(&mut self, f: impl FnOnce(&mut TimeCapsuleEventBuilder<S>) -> R) -> R {
        if self.num_of_active_events < self.events.len() {
            self.events.truncate(self.num_of_active_events)
        }
        let mut event_builder = TimeCapsuleEventBuilder::new(self);
        let r = f(&mut event_builder);
        if let Some(event) = event_builder.finish() {
            self.events.push(event);
            self.num_of_active_events += 1;
        }
        r
    }
}
