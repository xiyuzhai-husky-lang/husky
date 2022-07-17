use husky_entity_route::EntityRoutePtr;

use super::*;

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(super) fn call_specific_routine(
        &mut self,
        f: __SpecificRoutineLinkage,
        output_ty: EntityRoutePtr,
    ) -> __EvalResult<()> {
        let mut arguments = self.stack.drain(f.nargs).collect::<Vec<_>>();
        for argument in arguments.iter() {
            match argument {
                __TempValue::Moved => panic!(),
                _ => (),
            }
        }
        let output = f.call(self.opt_ctx, arguments)?;
        msg_once!("ugly");
        if output_ty.kind
            != (EntityRouteKind::Root {
                ident: RootIdentifier::DatasetType,
            })
        {
            should_eq!(
                output.ty(),
                output_ty,
                r#"
    linkage source: {:?}
    output:
        {output:?}
"#,
                f.dev_src
            );
        }
        self.stack.push(output.into());
        Ok(())
    }
    pub(super) fn call_generic_transfer(
        &mut self,
        output_ty: EntityRoutePtr,
        f: GenericRoutineLinkage,
    ) -> __EvalResult<()> {
        let mut parameters = self.stack.drain(f.nargs).collect::<Vec<_>>();
        let result = (f.call)(output_ty, &mut parameters);
        self.stack.push(result.into());
        Ok(())
    }

    pub(super) fn call_interpreted(
        &mut self,
        sheet: &InstructionSheet,
        nargs: u8,
        has_this: bool,
    ) -> __EvalResult<()> {
        let mut interpreter = Interpreter::new(
            self.db,
            self.opt_ctx,
            self.stack.drain(nargs),
            has_this,
            self.vm_config,
        );
        self.stack.push(
            interpreter
                .eval_instructions(sheet, Mode::Fast)?
                .into_stack()?,
        );
        Ok(())
    }
}
