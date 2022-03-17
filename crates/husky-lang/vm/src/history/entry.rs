use crate::*;

#[derive(Debug, Clone)]
pub enum HistoryEntry<'eval> {
    // Stmt { control: VMControlSnapshot },
    PureExpr {
        output: StackValueSnapshot<'eval>,
    },
    Exec,
    Assign {
        before: StackValueSnapshot<'eval>,
        after: StackValueSnapshot<'eval>,
    },
    Loop {
        result: ControlSnapshot<'eval>,
        stack_snapshot: StackSnapshot<'eval>,
        body: Arc<InstructionSheet>,
    },
}

impl<'eval> HistoryEntry<'eval> {
    pub fn value(&self) -> StackValueSnapshot<'eval> {
        match self {
            HistoryEntry::PureExpr { ref output } => output.clone(),
            HistoryEntry::Exec => todo!(),
            HistoryEntry::Loop { result, .. } => todo!(),
            HistoryEntry::Assign {
                ref before,
                ref after,
            } => after.clone(),
        }
    }

    pub(crate) fn loop_entry(
        result: &VMControl<'eval>,
        stack_snapshot: StackSnapshot<'eval>,
        body: Arc<InstructionSheet>,
    ) -> HistoryEntry<'eval> {
        HistoryEntry::Loop {
            result: result.snapshot(),
            stack_snapshot,
            body,
        }
    }
}
