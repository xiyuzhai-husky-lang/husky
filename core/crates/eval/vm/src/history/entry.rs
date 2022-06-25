use avec::Avec;

use crate::*;

#[derive(Debug, Clone)]
pub enum HistoryEntry<'eval> {
    // Stmt { control: VMControlSnapshot },
    PureExpr {
        output: EvalValueResult<'eval>,
    },
    Exec {
        mutations: Vec<MutationData<'eval>>,
    },
    Loop {
        loop_kind: VMLoopKind,
        control: ControlSnapshot<'eval>,
        stack_snapshot: StackSnapshot<'eval>,
        body_instruction_sheet: Arc<InstructionSheet>,
        mutations: Vec<MutationData<'eval>>,
    },
    ControlFlow {
        opt_branch_entered: Option<u8>,
        vm_branches: Avec<VMConditionBranch>,
        control: ControlSnapshot<'eval>,
        stack_snapshot: StackSnapshot<'eval>,
        mutations: Vec<MutationData<'eval>>,
    },
    PatternMatching {
        opt_branch_entered: Option<u8>,
        vm_branches: Avec<VMPatternBranch>,
        control: ControlSnapshot<'eval>,
        stack_snapshot: StackSnapshot<'eval>,
        mutations: Vec<MutationData<'eval>>,
    },
    Break,
}

impl<'eval> HistoryEntry<'eval> {
    pub fn value(&self) -> EvalValueResult<'eval> {
        match self {
            HistoryEntry::PureExpr { ref output } => output.clone(),
            HistoryEntry::Exec { mutations } => {
                if mutations.len() != 1 {
                    todo!()
                }
                Ok(mutations[0].after.clone())
            }
            HistoryEntry::Loop { .. } => todo!(),
            HistoryEntry::ControlFlow {
                opt_branch_entered: enter,
                ..
            } => todo!(),
            HistoryEntry::Break => todo!(),
            HistoryEntry::PatternMatching { .. } => todo!(),
        }
    }

    pub(crate) fn loop_entry(
        loop_kind: VMLoopKind,
        result: &VMControl<'eval>,
        stack_snapshot: StackSnapshot<'eval>,
        body: Arc<InstructionSheet>,
        mutations: Vec<MutationData<'eval>>,
    ) -> HistoryEntry<'eval> {
        HistoryEntry::Loop {
            loop_kind,
            control: result.snapshot(),
            stack_snapshot,
            body_instruction_sheet: body,
            mutations,
        }
    }
}
