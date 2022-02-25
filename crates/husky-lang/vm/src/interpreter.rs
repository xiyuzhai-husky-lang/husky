mod basic;

pub use basic::BasicInterpreter;

use crate::*;

pub trait Interpreter<'stack, 'eval: 'stack> {
    fn var(&self, rel_idx: usize) -> VMResult<&StackValue<'stack, 'eval>>;
    fn var_mut(&mut self, rel_idx: usize) -> VMResult<&mut StackValue<'stack, 'eval>>;
    fn len(&self) -> usize;
    fn push(&mut self, value: StackValue<'stack, 'eval>);
    fn pop(&mut self) -> VMResult<StackValue<'stack, 'eval>>;
    fn drain(&mut self, new_len: usize) -> Vec<StackValue<'stack, 'eval>>;

    fn exec_all(&mut self, instructions: &[Instruction]) -> VMResult<ControlSignal<'stack, 'eval>> {
        for ins in instructions {
            match self.exec(&ins.kind)? {
                ControlSignal::Normal => (),
                ControlSignal::Return(value) => return Ok(ControlSignal::Return(value)),
                ControlSignal::Break => return Ok(ControlSignal::Break),
            }
        }
        Ok(ControlSignal::Normal)
    }

    fn exec(&mut self, ins: &InstructionKind) -> VMResult<ControlSignal<'stack, 'eval>> {
        match ins {
            InstructionKind::PushVarInput(contract, rel_idx) => {
                self.push(self.var(*rel_idx as usize)?.as_input(*contract)?);
                Ok(ControlSignal::Normal)
            }
            InstructionKind::PushPrimitive(value) => {
                self.push(value.into());
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
            InstructionKind::Return => Ok(ControlSignal::Return(self.pop().unwrap())),
        }
    }

    fn finish(&mut self) -> VMResult<StackValue<'stack, 'eval>> {
        if self.len() != 1 {
            todo!()
        }
        Ok(self.pop().unwrap())
    }

    fn call(&mut self, f: &Compiled, nargs: u16) -> VMResult<()> {
        let new_len = self.len() - nargs as usize;
        let result = (f.call)(self.drain(new_len))?;
        self.push(result.into());
        Ok(())
    }

    fn exec_primitive(&mut self, opn: PrimitiveOpn) -> VMResult<()> {
        match opn {
            PrimitiveOpn::Binary(func) => {
                let ropd = self.pop().unwrap();
                let lopd = self.pop().unwrap();
                self.push(
                    func.act_on_primitives(lopd.as_primitive()?, ropd.as_primitive()?)?
                        .into(),
                );
                Ok(())
            }
            PrimitiveOpn::BinaryAssign { dst_idx, func } => {
                let ropd = self.pop().unwrap();
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
