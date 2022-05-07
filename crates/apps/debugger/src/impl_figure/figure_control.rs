use vm::LoopFrameData;

use super::*;

#[derive(Debug, Default, Serialize, Clone)]
#[serde(tag = "kind")]
pub struct FigureControl {
    opt_mutation_selection: Option<u8>,
}

impl FigureControl {
    pub fn loop_frame_default<'eval>(loop_frame_data: &LoopFrameData<'eval>) -> Self {
        FigureControl {
            opt_mutation_selection: if loop_frame_data.mutations.len() > 0 {
                Some(0)
            } else {
                None
            },
        }
    }
}
