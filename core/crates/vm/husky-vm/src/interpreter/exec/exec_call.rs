use husky_entity_route::EntityRoutePtr;

use super::*;

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(super) fn call_specific_routine(
        &mut self,
        f: __LinkageFp,
        nargs: u8,
        output_ty: EntityRoutePtr,
    ) -> __VMResult<()> {
        let mut arguments = self.stack.drain(nargs).collect::<Vec<_>>();
        for argument in arguments.iter() {
            todo!()
            // match argument {
            //     __TempValue::Moved => panic!(),
            //     _ => (),
            // }
        }
        let output = f.eval(self.opt_ctx, arguments)?;
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
    output:
        {output:?}
"#
            );
        }
        self.stack.push(output.into());
        Ok(())
    }

    pub(super) fn call_interpreted(
        &mut self,
        sheet: &InstructionSheet,
        nargs: u8,
        has_this: bool,
    ) -> __VMResult<()> {
        let mut interpreter = Interpreter::new(
            self.db,
            self.opt_ctx,
            self.stack.drain(nargs),
            has_this,
            self.vm_config,
        );
        self.stack
            .push(interpreter.eval_instructions(sheet, Mode::Fast)?.stack());
        Ok(())
    }

    pub(super) fn call_linkage(
        &mut self,
        linkage: __Linkage,
        nargs: u8,
        output_ty: EntityRoutePtr,
    ) -> __VMResult<()> {
        todo!()
    }
}
