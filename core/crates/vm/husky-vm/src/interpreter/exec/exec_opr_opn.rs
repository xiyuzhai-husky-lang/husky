use husky_entity_route::EntityRoutePtr;
use husky_print_utils::{epin, p};

use crate::*;

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(super) fn exec_opr_opn(
        &mut self,
        opn: OprOpn,
        debug_flag: Mode,
        ins: &Instruction,
    ) -> __VMResult<()> {
        match opn {
            OprOpn::RootPureBinary(pure_binary_opr) => {
                let ropd = self.stack.pop();
                let lopd = self.stack.pop();
                let output =
                    pure_binary_opr.act_on_primitives(lopd.primitive(), ropd.primitive())?;
                match debug_flag {
                    Mode::Fast | Mode::TrackMutation => (),
                    Mode::TrackHistory => self.history.write(
                        ins,
                        HistoryEntry::PureExpr {
                            result: Ok(output.to_register()),
                        },
                    ),
                }
                self.stack.push(output.to_register());
                Ok(())
            }
            OprOpn::RootBinaryAssign(opt_binary_opr) => {
                let ropd = self.stack.pop();
                let mut lopd = self.stack.pop();
                match debug_flag {
                    Mode::Fast | Mode::TrackMutation => {
                        binary_assign(opt_binary_opr, &mut lopd, ropd);
                    }
                    Mode::TrackHistory => {
                        let before = lopd.eval();
                        binary_assign(opt_binary_opr, &mut lopd, ropd);
                        let after = lopd.eval();
                        match ins.variant {
                            InstructionVariant::OprOpn {
                                this_ty,
                                this_range,
                                ..
                            } => self.history.write(
                                ins,
                                HistoryEntry::Exec {
                                    mutations: vec![MutationData {
                                        file: ins.src.file(),
                                        range: this_range,
                                        kind: MutationDataVariant::Exec,
                                        ty: this_ty,
                                        before: Some(before),
                                        after,
                                    }],
                                },
                            ),
                            _ => panic!(""),
                        }
                    }
                }
                Ok(())
            }
            OprOpn::Cast(cast_opn) => {
                let mut output = cast_opn.act_on(self.stack.pop());
                match debug_flag {
                    Mode::Fast | Mode::TrackMutation => (),
                    Mode::TrackHistory => self.history.write(
                        ins,
                        HistoryEntry::PureExpr {
                            result: Ok(output.eval()),
                        },
                    ),
                }
                self.stack.push(output);
                Ok(())
            }
            OprOpn::Prefix(prefix_opr) => {
                let output = prefix_opr.act_on_primitive(self.stack.pop().primitive());
                match debug_flag {
                    Mode::Fast | Mode::TrackMutation => (),
                    Mode::TrackHistory => self.history.write(
                        ins,
                        HistoryEntry::PureExpr {
                            result: Ok(output.to_register()),
                        },
                    ),
                }
                self.stack.push(output.to_register());
                Ok(())
            }
            OprOpn::Incr => {
                let mut opd = self.stack.pop();
                match debug_flag {
                    Mode::Fast | Mode::TrackMutation => {
                        incr(&mut opd);
                    }
                    Mode::TrackHistory => {
                        let before = opd.eval();
                        incr(&mut opd);
                        let after = opd.eval();
                        match ins.variant {
                            InstructionVariant::OprOpn {
                                this_ty,
                                this_range,
                                ..
                            } => self.history.write(
                                ins,
                                HistoryEntry::Exec {
                                    mutations: vec![MutationData {
                                        file: ins.src.file(),
                                        range: this_range,
                                        kind: MutationDataVariant::Exec,
                                        ty: this_ty,
                                        before: Some(before),
                                        after,
                                    }],
                                },
                            ),
                            _ => panic!(""),
                        }
                    }
                }
                Ok(())
            }
            OprOpn::Decr => {
                let mut opd = self.stack.pop();
                match debug_flag {
                    Mode::Fast | Mode::TrackMutation => {
                        decr(&mut opd);
                    }
                    Mode::TrackHistory => {
                        let before = opd.eval();
                        decr(&mut opd);
                        let after = opd.eval();
                        match ins.variant {
                            InstructionVariant::OprOpn {
                                this_ty,
                                this_range,
                                ..
                            } => self.history.write(
                                ins,
                                HistoryEntry::Exec {
                                    mutations: vec![MutationData {
                                        file: ins.src.file(),
                                        range: this_range,
                                        kind: MutationDataVariant::Exec,
                                        ty: this_ty,
                                        before: Some(before),
                                        after,
                                    }],
                                },
                            ),
                            _ => panic!(""),
                        }
                    }
                }
                Ok(())
            }
        }
    }
}
