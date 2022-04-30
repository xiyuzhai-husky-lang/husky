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
                let output = pure_binary_opr
                    .act_on_primitives(lopd.as_primitive()?, ropd.as_primitive()?)?;
                match debug_flag {
                    Mode::Fast | Mode::TrackMutation => (),
                    Mode::TrackHistory => self.history.write(
                        ins,
                        HistoryEntry::NonVoidExpr {
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
                let lopd_value = lopd.as_primitive()?;
                match lopd {
                    StackValue::MutLocalRef { ref mut value, .. } => {
                        value.assign(if let Some(binary_opr) = opt_binary_opr {
                            binary_opr
                                .act_on_primitives(lopd_value, ropd.as_primitive()?)?
                                .into()
                        } else {
                            ropd
                        });
                    }
                    _ => {
                        p!(lopd);
                        panic!()
                    }
                }
                let after = lopd.snapshot();
                match debug_flag {
                    Mode::Fast | Mode::TrackMutation => (),
                    Mode::TrackHistory => self
                        .history
                        .write(ins, HistoryEntry::Assign { before, after }),
                }
                Ok(())
            }
            PrimitiveOpn::Unary => todo!(),
        }
    }
}
