use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_lib_rs_content(&mut self) {
        let package = self.package();
        self.write("#![allow(warnings)]\n");
        self.write("pub mod __init__;\n");
        self.write("use __husky::*;\n\n");
        self.gen_mod_rs_content(&package.subentities);
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }
}
