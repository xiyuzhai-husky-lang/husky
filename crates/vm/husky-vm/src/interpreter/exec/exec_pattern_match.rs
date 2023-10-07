use crate::*;

impl<'temp> Interpreter<'temp> {
    pub(super) fn exec_pattern_matching(
        &mut self,
        ins: &Instruction,
        branches: &[VMPatternBranch],
        mode: Mode,
    ) -> VMControl {
        let stack_len = self.stack.len();
        let mut indexed_branch_iter = branches.iter().enumerate();
        let match_expr = self.stack.pop();
        let opt_indexed_branch_entered = loop {
            match indexed_branch_iter.next() {
                Some((i, b)) => {
                    let enter: bool = if let Some(ref pattern) = b.opt_pattern {
                        pattern.contains(&match_expr)
                    } else {
                        true
                    };
                    // if let Some(ref condition) = b.opt_condition_sheet {
                    //     self.exec_all(condition, mode); // compute condition
                    //     self.stack.pop().to_bool()
                    // } else {
                    //     true
                    // };
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
                        self.save_snapshot("Pattern Matching".to_string());
                        let control = self.exec_all(&b.body, Mode::TrackHistory);
                        let (stack_snapshot, mutations) = self.collect_block_mutations();
                        self.history.write(
                            ins,
                            HistoryEntry::PatternMatching {
                                opt_branch_entered: Some(i.try_into().unwrap()),
                                stack_snapshot,
                                control: control.snapshot(),
                                mutations,
                                vm_branches: todo!(),
                                //  branches.clone(),
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
                        HistoryEntry::PatternMatching {
                            opt_branch_entered: None,
                            stack_snapshot: self.stack.snapshot(format!("Pattern matching")),
                            control: ControlSnapshot::None,
                            mutations: Vec::new(),
                            vm_branches: todo!(),
                            // branches.clone(),
                        },
                    );
                }
                VMControl::None
            }
        }
    }
}
