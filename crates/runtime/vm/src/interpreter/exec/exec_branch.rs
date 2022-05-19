use avec::Avec;
use check_utils::should;
use print_utils::p;

use crate::*;

impl<'stack, 'eval: 'stack> Interpreter<'stack, 'eval> {
    pub(super) fn exec_branch(
        &mut self,
        sheet: &InstructionSheet,
        ins: &Instruction,
        branches: &Avec<VMConditionBranch>,
        mode: Mode,
    ) -> VMControl<'eval> {
        should!(self.stack.len() <= sheet.variable_stack.len());
        let stack_len = self.stack.len();
        let mut indexed_branch_iter = branches.iter().enumerate();
        let opt_indexed_branch_entered = loop {
            match indexed_branch_iter.next() {
                Some((i, b)) => {
                    let enter: bool = if let Some(ref condition) = b.opt_condition_sheet {
                        self.exec_all(condition, mode); // compute condition
                        self.stack.pop().to_bool()
                    } else {
                        true
                    };
                    if enter {
                        break Some((i, b));
                    }
                }
                None => break None,
            }
        };
        match opt_indexed_branch_entered {
            Some((i, b)) => {
                let control = match mode {
                    Mode::Fast => self.exec_all(&b.body, Mode::Fast),
                    Mode::TrackMutation => self.exec_all(&b.body, Mode::TrackMutation),
                    Mode::TrackHistory => {
                        self.save_snapshot();
                        let control = self.exec_all(&b.body, Mode::TrackHistory);
                        let (stack_snapshot, mutations) = self.collect_block_mutations();
                        self.history.write(
                            ins,
                            HistoryEntry::BranchGroup {
                                opt_branch_entered: Some(i.try_into().unwrap()),
                                stack_snapshot,
                                branches: branches.clone(),
                                control: control.snapshot(),
                                mutations,
                                vm_branches: branches.clone(),
                            },
                        );
                        control
                    }
                };
                self.stack.truncate(stack_len);
                control
            }
            None => {
                if mode == Mode::TrackHistory {
                    self.history.write(
                        ins,
                        HistoryEntry::BranchGroup {
                            opt_branch_entered: None,
                            stack_snapshot: self.stack.snapshot(),
                            branches: branches.clone(),
                            control: ControlSnapshot::None,
                            mutations: Vec::new(),
                            vm_branches: branches.clone(),
                        },
                    );
                }
                VMControl::None
            }
        }
    }
}
