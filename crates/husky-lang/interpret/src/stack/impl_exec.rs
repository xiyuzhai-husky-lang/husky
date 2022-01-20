use common::*;

use crate::*;

impl<'stack> VirtualStack<'stack> {
    pub fn exec_all(&mut self, instructions: &[Instruction]) -> InterpretResult<ControlSignal> {
        for ins in instructions {
            p!(ins);
            match self.exec(&ins.kind)? {
                ControlSignal::Normal => (),
                ControlSignal::Return => return Ok(ControlSignal::Return),
                ControlSignal::Break => return Ok(ControlSignal::Break),
            }
        }
        Ok(ControlSignal::Normal)
    }

    pub fn exec(&mut self, ins: &InstructionKind) -> InterpretResult<ControlSignal> {
        match ins {
            InstructionKind::PushVarInput(contract, rel_idx) => {
                self.push(self.var(*rel_idx as usize)?.as_input(*contract)?)?;
                Ok(ControlSignal::Normal)
            }
            InstructionKind::PushPrimitive(value) => {
                self.push(value.into())?;
                Ok(ControlSignal::Normal)
            }
            InstructionKind::Call { compiled, nargs } => {
                self.call(compiled, *nargs)?;
                Ok(ControlSignal::Normal)
            }
            InstructionKind::Primitive(opn) => {
                self.exec_primitive(*opn)?;
                Ok(ControlSignal::Normal)
            }
            InstructionKind::CallInterpret(instructions) => self.exec_all(&instructions),
            InstructionKind::Return => {
                self.ret()?;
                Ok(ControlSignal::Return)
            }
        }
    }

    pub fn ret(&mut self) -> InterpretResult<()> {
        if self.len() >= 2 + self.current_frame_start {
            let result = self.values.pop().unwrap();
            self.values.truncate(self.current_frame_start);
            self.values.push(result);
        }
        Ok(())
    }
}

impl<'stack> VirtualStack<'stack> {
    pub fn push(&mut self, item: StackValue<'stack>) -> InterpretResult<()> {
        self.values.push(item);
        if self.len() >= self.stack_size {
            todo!()
        }
        Ok(())
    }

    pub fn finish(&mut self) -> InterpretResult<StackValue> {
        if self.len() != 1 {
            p!(self.len());
            todo!()
        }
        Ok(self.values.pop().unwrap())
    }
}

impl<'stack> VirtualStack<'stack> {
    fn call(&mut self, f: &Compiled, nargs: u16) -> InterpretResult<()> {
        epin!();
        let save = self.current_frame_start;
        // take nargs
        self.current_frame_start = self.len() - (nargs as usize);
        (f.0)(self)?;
        p!(self.len());
        self.current_frame_start = save;
        Ok(())
    }

    fn exec_primitive(&mut self, opn: PrimitiveOpn) -> InterpretResult<()> {
        match opn {
            PrimitiveOpn::Binary(func) => {
                let ropd = self.values.pop().unwrap();
                let lopd = self.values.pop().unwrap();
                self.values.push(
                    func.call(lopd.as_primitive()?, ropd.as_primitive()?)?
                        .into(),
                );
                Ok(())
            }
            PrimitiveOpn::BinaryAssign { dst_idx, func } => {
                let ropd = self.values.pop().unwrap();
                let lopd = self.var_mut(dst_idx as usize)?;
                *lopd = func
                    .call(lopd.as_primitive()?, ropd.as_primitive()?)?
                    .into();
                Ok(())
            }
            PrimitiveOpn::Unary => todo!(),
        }
    }
}
