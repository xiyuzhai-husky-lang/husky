use crate::*;

#[derive(Debug, Clone)]
pub enum HistoryEntry<'eval> {
    // Stmt { control: VMControlSnapshot },
    NonVoidExpr {
        output: StackValueSnapshot<'eval>,
    },
    Exec,
    Assign {
        before: StackValueSnapshot<'eval>,
        after: StackValueSnapshot<'eval>,
    },
    Loop {
        control: ControlSnapshot<'eval>,
        stack_snapshot: StackSnapshot<'eval>,
        body: Arc<InstructionSheet>,
        mutations: Vec<MutationData<'eval>>,
    },
}

impl<'eval> HistoryEntry<'eval> {
    pub fn value(&self) -> StackValueSnapshot<'eval> {
        match self {
            HistoryEntry::NonVoidExpr { ref output } => output.clone(),
            HistoryEntry::Exec => todo!(),
            HistoryEntry::Loop {
                control: result, ..
            } => todo!(),
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
        mutations: Vec<MutationData<'eval>>,
    ) -> HistoryEntry<'eval> {
        HistoryEntry::Loop {
            control: result.snapshot(),
            stack_snapshot,
            body,
            mutations,
        }
    }
}
