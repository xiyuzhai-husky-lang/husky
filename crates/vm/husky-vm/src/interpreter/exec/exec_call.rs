use super::*;

impl<'temp> Interpreter<'temp> {
    pub(super) fn call_specific_routine(
        &mut self,
        f: __ResolvedLinkage,
        nargs: u8,
        discard: bool,
    ) -> __VMResult<()> {
        let arguments = self.stack.drain(nargs).collect::<Vec<_>>();
        for argument in arguments.iter() {
            if self.stack.len() > 0 {
                assert_ne!(argument.vtable as *const _, &__VOID_VTABLE as *const _);
            }
            match argument.data_kind() {
                __RegisterDataKind::Moved | __RegisterDataKind::Unreturned => panic!(),
                __RegisterDataKind::SomeNone => todo!(),
                _ => (),
            }
        }
        let output = f.call_catch_unwind(self.opt_ctx, arguments)?;
        if !discard {
            self.stack.push(output);
        }
        Ok(())
    }

    pub(super) fn call_interpreted(
        &mut self,
        sheet: &Instructions,
        nargs: u8,
        discard: bool,
    ) -> __VMResult<()> {
        let mut interpreter = Interpreter::new(
            self.db,
            self.opt_ctx,
            self.stack.drain(nargs),
            self.vm_config,
        );
        if !discard {
            self.stack
                .push(interpreter.eval_instructions(sheet, Mode::Fast)?);
        }
        Ok(())
    }
}
