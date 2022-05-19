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
            OprOpn::Assign(opt_binary_opr) => {
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
                        p!(ins.src.text_range());
                        p!(self.stack);
                        panic!()
                    }
                }
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
            OprOpn::Suffix(suffix_opr) => {
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
            OprOpn::Prefix(prefix_opr) => {
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
