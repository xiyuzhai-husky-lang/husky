use super::*;

pub struct MathAstTextEditAction<F>
where
    F: FnOnce(&mut String),
{
    ast_idx: MathAstIdx,
    f: F,
}

impl<F> MathAstTextEditAction<F>
where
    F: FnOnce(&mut String),
{
    pub fn new(ast_idx: MathAstIdx, f: F) -> Self {
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

    fn exec(self, sheet: &mut MathAstSheet) -> Self::Outcome {
        sheet.arena.update(self.ast_idx, |ast| match ast {
            MathAstData::TextEdit { ref mut buffer, .. } => (self.f)(buffer),
            MathAstData::Other => unreachable!("shouldn't use this"),
        })
    }
}

#[test]
fn math_ast_text_edit_action_works() {
    use crate::ast::MathAstTextEditKind;

    let mut sheet: MathAstSheet = Default::default();
    let ast_idx = sheet.alloc_ast(MathAstData::TextEdit {
        kind: MathAstTextEditKind::Latex,
        buffer: "hello,".to_string(),
    });
    let action = MathAstTextEditAction::new(ast_idx, |s| *s += " world");
    action.exec(&mut sheet);
    let MathAstData::TextEdit { ref buffer, .. } = sheet.arena[ast_idx] else {
        unreachable!()
    };
    assert_eq!(buffer, "hello, world")
}
