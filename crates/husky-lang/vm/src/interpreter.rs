mod exec;

use crate::*;

pub struct Interpreter<'stack, 'eval: 'stack> {
    stack: VMStack<'stack, 'eval>,
    pub(crate) history: History<'eval>,
    snapshot: Option<StackSnapshot<'eval>>,
    pub(crate) frames: Vec<LoopFrameSnapshot<'eval>>,
}

impl<'stack, 'eval: 'stack> Interpreter<'stack, 'eval> {
    pub(crate) fn try_new(
        iter: impl Iterator<Item = VMResult<StackValue<'stack, 'eval>>>,
    ) -> VMResult<Interpreter<'stack, 'eval>> {
        Ok(Self {
            stack: VMStack::try_new(iter)?,
            history: Default::default(),
            snapshot: None,
            frames: vec![],
        })
    }
    // Vec<StackValue<'stack, 'eval>>
    pub(crate) fn new(values: impl Into<VMStack<'stack, 'eval>>) -> Interpreter<'stack, 'eval> {
        Self {
            stack: values.into(),
            history: Default::default(),
            snapshot: None,
            frames: vec![],
        }
    }

    pub(crate) fn eval_instructions(
        &mut self,
        instructions: &[Instruction],
        mode: Mode,
    ) -> EvalResult<'eval> {
        match self.exec_all(instructions, mode) {
            VMControl::None => {
                panic!("no return from eval_instructions")
            }
            VMControl::Return(result) => Ok(result),
            VMControl::Break => todo!(),
            VMControl::Err(_) => todo!(),
        }
    }

    // fn finish(&mut self) -> VMResult<StackValue<'stack, 'eval>> {
    //     if self.stack.len() != 1 {
    //         todo!()
    //     }
    //     Ok(self.stack.pop())
    // }

    fn call_compiled(&mut self, f: &Compiled, nargs: u8) -> VMResult<()> {
        let result = (f.call)(self.stack.topk_mut(nargs))?;
        self.stack.push(result.into());
        Ok(())
    }

    fn routine_call_interpreted(
        &mut self,
        instructions: &[Instruction],
        nargs: u8,
    ) -> VMResult<()> {
        todo!()
    }

    fn ty_call_interpreted(&mut self, signature: &VMTySignature) -> VMResult<()> {
        match signature {
            VMTySignature::Enum => todo!(),
            VMTySignature::Struct { ref memb_vars } => {
                let inputs = self.stack.drain(memb_vars.len().try_into().unwrap());
                self.stack
                    .push(VirtualTy::new_struct(inputs, memb_vars).into());
                Ok(())
            }
        }
    }

    fn take_snapshot(&mut self) {
        if let Some(_) = self.snapshot {
            panic!()
        }
        self.snapshot = Some(self.stack.snapshot());
    }

    fn take_changes(&mut self) -> (StackSnapshot<'eval>, Vec<()>) {
        if let Some(snapshot) = std::mem::take(&mut self.snapshot) {
            (snapshot, vec![()])
        } else {
            panic!()
        }
    }
}
