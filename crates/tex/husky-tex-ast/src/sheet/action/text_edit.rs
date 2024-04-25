use super::*;
use crate::data::{math::TexMathAstData, rose::TexRoseAstData};

pub(in crate::sheet) struct MathAstTextEditAction<F>
where
    F: FnOnce(&mut String),
{
    ast_idx: TexAstIdx,
    f: F,
}

impl<F> MathAstTextEditAction<F>
where
    F: FnOnce(&mut String),
{
    pub(in crate::sheet) fn new(ast_idx: TexAstIdx, f: F) -> Self {
        Self { ast_idx, f }
    }
}

impl<F> IsTimeCapsuleAction for MathAstTextEditAction<F>
where
    F: FnOnce(&mut String),
{
    type Event = MathAstEvent;

    type Outcome = ();

    fn add_to_event_buffer(&self, _event: &mut MathAstEventBuffer) {
        // ignored because text editing is too trivial to be recorded
        ()
    }

    fn exec(self, sheet: &mut TexAstSheet) -> Self::Outcome {
        sheet.arena.update(self.ast_idx, |ast| match ast {
            TexAstData::Math(TexMathAstData::TextEdit { ref mut buffer, .. }) => (self.f)(buffer),
            TexAstData::Rose(TexRoseAstData::TextEdit { ref mut buffer, .. }) => (self.f)(buffer),
            _ => unreachable!("shouldn't use this"),
        })
    }
}

#[test]
fn math_ast_text_edit_action_works() {
    let mut sheet: TexAstSheet = Default::default();
    let ast_idx = sheet.alloc_ast(
        TexMathAstData::TextEdit {
            buffer: "hello,".to_string(),
        }
        .into(),
    );
    let action = MathAstTextEditAction::new(ast_idx, |s| *s += " world");
    action.exec(&mut sheet);
    let TexAstData::Math(TexMathAstData::TextEdit { ref buffer, .. }) = sheet.arena[ast_idx] else {
        unreachable!()
    };
    assert_eq!(buffer, "hello, world")
}
