use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_lib_rs_content(&mut self) {
        let package = self.package();
        let crate_input_ty = self.db.crate_input_ty(package.main_file()).unwrap();
        self.write(&format!(
            r#"#![allow(warnings)]
pub mod __init__;
pub mod __registration__;
use __husky::root::*;

fn __input<'a, 'eval: 'a>(
    __ctx: &'a dyn __EvalContext<'eval>
) -> &'a "#,
        ));
        self.gen_entity_route(crate_input_ty, EntityRouteRole::Decl);
        let mangled_crate_input_ty_vtable = self.db.mangled_ty_vtable(crate_input_ty);
        self.write(&format!(
            r#" {{
    unsafe {{ __ctx.crate_input().downcast_temp_ref(&__registration__::{mangled_crate_input_ty_vtable}) }}
}}
"#
        ));
        self.gen_mod_rs_content(&package.subentities);
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }
}
