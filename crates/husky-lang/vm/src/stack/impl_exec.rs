use common::*;

use crate::*;

impl<'stack, 'eval: 'stack> VMStack<'stack, 'eval> {
    pub fn exec_all(
        &mut self,
        instructions: &[Instruction],
    ) -> VMResult<ControlSignal<'stack, 'eval>> {
        for ins in instructions {
            match self.exec(&ins.kind)? {
                ControlSignal::Normal => (),
                ControlSignal::Return(value) => return Ok(ControlSignal::Return(value)),
                ControlSignal::Break => return Ok(ControlSignal::Break),
            }
        }
        Ok(ControlSignal::Normal)
    }

    pub fn exec(&mut self, ins: &InstructionKind) -> VMResult<ControlSignal<'stack, 'eval>> {
        match ins {
            InstructionKind::PushVarInput(contract, rel_idx) => {
                self.values
                    .push(self.var(*rel_idx as usize)?.as_input(*contract)?);
                Ok(ControlSignal::Normal)
            }
            InstructionKind::PushPrimitive(value) => {
                self.values.push(value.into());
                Ok(ControlSignal::Normal)
            }
            InstructionKind::Call { compiled, nargs } => {
                self.call(compiled, *nargs)?;
                Ok(ControlSignal::Normal)
            }
            InstructionKind::PrimitiveOpn(opn) => {
                self.exec_primitive(*opn)?;
                Ok(ControlSignal::Normal)
            }
            InstructionKind::CallInterpret(instructions) => self.exec_all(&instructions),
            InstructionKind::Return => Ok(ControlSignal::Return(self.values.pop().unwrap())),
        }
    }
}

impl<'stack, 'eval: 'stack> VMStack<'stack, 'eval> {
    pub fn finish(&mut self) -> VMResult<StackValue<'stack, 'eval>> {
        if self.len() != 1 {
            p!(self.len());
            todo!()
        }
        Ok(self.values.pop().unwrap())
    }
}

impl<'stack, 'eval: 'stack> VMStack<'stack, 'eval> {
    fn call(&mut self, f: &Compiled, nargs: u16) -> VMResult<()> {
        let new_len = self.values.len() - nargs as usize;
        let result = (f.call)(self.values.drain(new_len..).collect())?;
        self.values.push(result.into());
        Ok(())
    }

    fn exec_primitive(&mut self, opn: PrimitiveOpn) -> VMResult<()> {
        match opn {
            PrimitiveOpn::Binary(func) => {
                let ropd = self.values.pop().unwrap();
                let lopd = self.values.pop().unwrap();
                self.values.push(
                    func.act_on_primitives(lopd.as_primitive()?, ropd.as_primitive()?)?
                        .into(),
                );
                Ok(())
            }
            PrimitiveOpn::BinaryAssign { dst_idx, func } => {
                let ropd = self.values.pop().unwrap();
                let lopd = self.var_mut(dst_idx as usize)?;
                *lopd = func
                    .act_on_primitives(lopd.as_primitive()?, ropd.as_primitive()?)?
                    .into();
                Ok(())
            }
            PrimitiveOpn::Unary => todo!(),
        }
    }
}
