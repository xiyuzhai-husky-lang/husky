use super::*;
use time_capsule::{
    capsule::TimeCapsule,
    event::{IsTimeCapsuleEvent, IsTimeCapsuleEventBuffer},
};

pub struct MathAstEvent {}

impl IsTimeCapsuleEvent for MathAstEvent {
    type Buffer = MathAstEventBuffer;

    type State = TexAstSheet;
}

#[derive(Default)]
pub struct MathAstEventBuffer {}

impl IsTimeCapsuleEventBuffer for MathAstEventBuffer {
    type Event = MathAstEvent;

    fn finish(self) -> Option<Self::Event> {
        todo!()
    }
}
