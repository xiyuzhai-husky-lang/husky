use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_lib_rs_content(&mut self) {
        todo!()
        //         let package = self.package();
        //         let target_parameter_ty = self.db.target_parameter_ty().unwrap();
        //         self.write(&format!(
        //             r#"#![allow(warnings)]
        // pub mod __init__;
        // pub mod __registration__;
        // use __husky::root::*;

        // fn __input<'a, 'eval: 'a>(
        //     __ctx: &'a dyn __EvalContext<'eval>
        // ) -> &'a "#,
        //         ));
        //         self.gen_entity_route(target_parameter_ty, EntityRouteRole::Decl);
        //         let mangled_target_parameter_ty_vtable = self.db.mangled_intrinsic_ty_vtable(target_parameter_ty);
        //         self.write(&format!(
        //             r#" {{
        //     unsafe {{ __ctx.target_input().downcast_temp_ref(&__registration__::{mangled_target_parameter_ty_vtable}) }}
        // }}
        // "#
        //         ));
        //         self.gen_mod_rs_content(&package.subentities);
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }
}
