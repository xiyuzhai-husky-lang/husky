use super::*;

impl<'vm, 'eval: 'vm> Interpreter<'vm, 'eval> {
    pub(super) fn call_compiled(&mut self, f: Linkage) -> VMRuntimeResult<()> {
        let result = (f.call)(&mut self.stack.drain(f.nargs).collect::<Vec<_>>())?;
        self.stack.push(result.into());
        Ok(())
    }

    pub(super) fn call_interpreted(
        &mut self,
        sheet: &InstructionSheet,
        nargs: u8,
        has_this: bool,
    ) -> VMRuntimeResult<()> {
        let mut interpreter = Interpreter::new(self.db, self.stack.drain(nargs), has_this);
        self.stack.push(
            interpreter
                .eval_instructions(sheet, Mode::Fast)?
                .into_stack()?,
        );
        Ok(())
    }
}
