use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_lib_rs_content(&mut self) {
        let package = self.package();
        self.write(
            r#"#![allow(warnings)]
pub mod __init__;
pub mod __registration__;
use __husky::root::*;

"#,
        );
        self.gen_mod_rs_content(&package.subentities);
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }
}
