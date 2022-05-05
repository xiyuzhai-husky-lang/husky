use entity_route::EntityRoutePtr;
use print_utils::p;

use crate::*;

impl<'stack, 'eval: 'stack> Interpreter<'stack, 'eval> {
    pub(super) fn exec_primitive_opn(
        &mut self,
        opn: PrimitiveOpn,
        debug_flag: Mode,
        ins: &Instruction,
    ) -> VMResult<()> {
        match opn {
            PrimitiveOpn::PureBinary(pure_binary_opr) => {
                let ropd = self.stack.pop();
                let lopd = self.stack.pop();
                let output =
                    pure_binary_opr.act_on_primitives(lopd.as_primitive(), ropd.as_primitive())?;
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
            PrimitiveOpn::Assign(opt_binary_opr) => {
                let ropd = self.stack.pop();
                let mut lopd = self.stack.pop();
                let before = lopd.snapshot();
                let lopd_value = lopd.as_primitive();
                match lopd {
                    StackValue::LocalRefMut { ref mut value, .. } => {
                        value.assign(if let Some(binary_opr) = opt_binary_opr {
                            binary_opr
                                .act_on_primitives(lopd_value, ropd.as_primitive())?
                                .into()
                        } else {
                            ropd
                        });
                    }
                    _ => {
                        p!(lopd);
                        p!(ins.src);
                        panic!()
                    }
                }
                let after = lopd.snapshot();
                match debug_flag {
                    Mode::Fast | Mode::TrackMutation => (),
                    Mode::TrackHistory => match ins.kind {
                        InstructionKind::PrimitiveOpn {
                            this_ty,
                            this_range,
                            ..
                        } => self.history.write(
                            ins,
                            HistoryEntry::Exec {
                                mutations: vec![MutationData {
                                    file: ins.src.file(),
                                    range: this_range,
                                    ty: this_ty,
                                    before,
                                    after,
                                }],
                            },
                        ),
                        _ => panic!(""),
                    },
                }
                Ok(())
            }
            PrimitiveOpn::Suffix(suffix_opr) => {
                let output = suffix_opr.act_on_primitive(self.stack.pop().as_primitive());
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
            PrimitiveOpn::Prefix(prefix_opr) => {
                let output = prefix_opr.act_on_primitive(self.stack.pop().as_primitive());
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
