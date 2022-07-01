use husky_entity_route_syntax::EntityRoutePtr;

use super::*;

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(super) fn call_routine(&mut self, f: SpecificRoutineLinkage) -> EvalResult<()> {
        let mut parameters = self.stack.drain(f.nargs).collect::<Vec<_>>();
        let result = (f.call)(&mut parameters)?;
        self.stack.push(result.into());
        Ok(())
    }
    pub(super) fn call_generic_transfer(
        &mut self,
        output_ty: EntityRoutePtr,
        f: GenericRoutineLinkage,
    ) -> EvalResult<()> {
        let mut parameters = self.stack.drain(f.nargs).collect::<Vec<_>>();
        let result = (f.call)(output_ty, &mut parameters)?;
        self.stack.push(result.into());
        Ok(())
    }

    pub(super) fn call_interpreted(
        &mut self,
        sheet: &InstructionSheet,
        nargs: u8,
        has_this: bool,
    ) -> EvalResult<()> {
        let mut interpreter =
            Interpreter::new(self.db, self.stack.drain(nargs), has_this, self.verbose);
        self.stack.push(
            interpreter
                .eval_instructions(sheet, Mode::Fast)?
                .into_stack()?,
        );
        Ok(())
    }
}
