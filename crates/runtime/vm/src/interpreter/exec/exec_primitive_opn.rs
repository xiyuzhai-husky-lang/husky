use entity_route::EntityRoutePtr;
use print_utils::{epin, p};

use crate::*;

impl<'stack, 'eval: 'stack> Interpreter<'stack, 'eval> {
    pub(super) fn exec_primitive_opn(
        &mut self,
        opn: OprOpn,
        debug_flag: Mode,
        ins: &Instruction,
    ) -> VMRuntimeResult<()> {
        match opn {
            OprOpn::PureBinary(pure_binary_opr) => {
                let ropd = self.stack.pop();
                let lopd = self.stack.pop();
                let output =
                    pure_binary_opr.act_on_primitives(lopd.primitive(), ropd.primitive())?;
                match debug_flag {
                    Mode::Fast | Mode::TrackMutation => (),
                    Mode::TrackHistory => self.history.write(
                        ins,
                        HistoryEntry::PureExpr {
                            output: output.into(),
                        },
                    ),
                }
                self.stack.push(output.into());
                Ok(())
            }
            OprOpn::Assign(assign_opn) => {
                let ropd = self.stack.pop();
                let mut lopd = self.stack.pop();
                let before = lopd.snapshot();
                let lopd_value = lopd.primitive();
                assign_opn.act_on(&mut lopd, ropd);
                let after = lopd.snapshot();
                match debug_flag {
                    Mode::Fast | Mode::TrackMutation => (),
                    Mode::TrackHistory => match ins.kind {
                        InstructionKind::OprOpn {
                            this_ty,
                            this_range,
                            ..
                        } => self.history.write(
                            ins,
                            HistoryEntry::Exec {
                                mutations: vec![MutationData {
                                    file: ins.src.file(),
                                    kind: MutationDataKind::Exec { range: this_range },
                                    ty: this_ty,
                                    before: Some(before),
                                    after,
                                }],
                            },
                        ),
                        _ => panic!(""),
                    },
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
                            output: output.snapshot(),
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
                            output: output.into(),
                        },
                    ),
                }
                self.stack.push(output.into());
                Ok(())
            }
        }
    }
}
