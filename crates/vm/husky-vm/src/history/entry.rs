use avec::Avec;

use crate::*;

#[derive(Debug)]
pub enum HistoryEntry<'eval> {
    PureExpr {
        result: __VMResult<__Register<'eval>>,
        ty: EtherealTerm,
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
    pub fn result(&self) -> __VMResult<__Register<'eval>> {
        match self {
            HistoryEntry::PureExpr { ref result, .. } => result.clone(),
            HistoryEntry::Exec { mutations } => {
                if mutations.len() != 1 {
                    todo!()
                }
                Ok(mutations[0].after.clone())
            }
            HistoryEntry::Loop { .. } => todo!(),
            HistoryEntry::ControlFlow { .. } => todo!(),
            HistoryEntry::Break => todo!(),
            HistoryEntry::PatternMatching { .. } => todo!(),
        }
    }

    pub(crate) fn loop_entry(
        loop_kind: VMLoopKind,
        control: &VMControl<'eval>,
        stack_snapshot: StackSnapshot<'eval>,
        body: Arc<InstructionSheet>,
        mutations: Vec<MutationData<'eval>>,
    ) -> HistoryEntry<'eval> {
        HistoryEntry::Loop {
            loop_kind,
            control: control.snapshot(),
            stack_snapshot,
            body_instruction_sheet: body,
            mutations,
        }
    }
}
