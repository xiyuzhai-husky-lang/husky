use crate::*;

#[derive(Debug, Clone)]
pub enum HistoryEntry<'eval> {
    // Stmt { control: VMControlSnapshot },
    PureExpr {
        output: StackValueSnapshot<'eval>,
    },
    Exec {
        mutations: Vec<MutationData<'eval>>,
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
            HistoryEntry::PureExpr { ref output } => output.clone(),
            HistoryEntry::Exec { mutations } => {
                if mutations.len() != 1 {
                    todo!()
                }
                mutations[0].after.clone()
            }
            HistoryEntry::Loop { .. } => todo!(),
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
