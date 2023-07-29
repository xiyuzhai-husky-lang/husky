use avec::Avec;

use crate::*;

#[derive(Debug)]
pub enum HistoryEntry {
    PureExpr {
        result: __VMResult<__RegularValue>,
        ty: EtherealTerm,
    },
    Exec {
        mutations: Vec<MutationData>,
    },
    Loop {
        loop_kind: VMLoopKind,
        control: ControlSnapshot,
        stack_snapshot: StackSnapshot,
        body_instruction_sheet: Instructions,
        mutations: Vec<MutationData>,
    },
    ControlFlow {
        opt_branch_entered: Option<u8>,
        vm_branches: Avec<VMConditionBranch>,
        control: ControlSnapshot,
        stack_snapshot: StackSnapshot,
        mutations: Vec<MutationData>,
    },
    PatternMatching {
        opt_branch_entered: Option<u8>,
        vm_branches: Avec<VMPatternBranch>,
        control: ControlSnapshot,
        stack_snapshot: StackSnapshot,
        mutations: Vec<MutationData>,
    },
    Break,
}

impl HistoryEntry {
    pub fn result(&self) -> __VMResult<__RegularValue> {
        todo!()
        // match self {
        //     HistoryEntry::PureExpr { ref result, .. } => result.clone(),
        //     HistoryEntry::Exec { mutations } => {
        //         if mutations.len() != 1 {
        //             todo!()
        //         }
        //         Ok(mutations[0].after.clone())
        //     }
        //     HistoryEntry::Loop { .. } => todo!(),
        //     HistoryEntry::ControlFlow { .. } => todo!(),
        //     HistoryEntry::Break => todo!(),
        //     HistoryEntry::PatternMatching { .. } => todo!(),
        // }
    }

    pub(crate) fn loop_entry(
        loop_kind: VMLoopKind,
        control: &VMControl,
        stack_snapshot: StackSnapshot,
        body: Instructions,
        mutations: Vec<MutationData>,
    ) -> HistoryEntry {
        HistoryEntry::Loop {
            loop_kind,
            control: control.snapshot(),
            stack_snapshot,
            body_instruction_sheet: body,
            mutations,
        }
    }
}
