use crate::*;

#[derive(Debug, Clone)]
pub enum HistoryEntry {
    // Stmt { control: VMControlSnapshot },
    PureExpr {
        output: StackValueSnapshot,
    },
    Exec,
    Assign {
        before: StackValueSnapshot,
        after: StackValueSnapshot,
    },
    Loop {
        result: ControlSnapshot,
        stack_snapshot: StackSnapshot,
        body: Arc<InstructionSheet>,
    },
}

impl HistoryEntry {
    pub fn value(&self) -> StackValueSnapshot {
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

    pub(crate) fn loop_entry<'eval>(
        result: &VMControl<'eval>,
        stack_snapshot: StackSnapshot,
        body: Arc<InstructionSheet>,
    ) -> HistoryEntry {
        HistoryEntry::Loop {
            result: result.snapshot(),
            stack_snapshot,
            body,
        }
    }
}
