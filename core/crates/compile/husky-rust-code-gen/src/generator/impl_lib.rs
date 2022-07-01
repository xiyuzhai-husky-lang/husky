use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_package_lib_rs_content(&mut self) {
        let package = self.package();
        self.write("#![allow(warnings)]\n");
        self.write("use __root::*;\n");
        self.gen_mod_rs_content(&package.subentities);
        self.gen_init();
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }

    fn gen_init(&mut self) {
        emsg_once!("link entity with compiled");
        self.result += r#"
pub mod __init__ {
    pub fn link_entity_with_compiled(compile_time: &mut husky_compile_time::HuskyCompileTime) {
"#;
        let main_module = self.db.module(self.package_main).unwrap();
        let entity_dependees = self.db.entity_dependees(main_module).unwrap();
        for (entity_route, _) in entity_dependees.iter() {
            self.result += &format!("        todo!(\"{:?}\");\n", entity_route);
        }
        self.result += "    }\n}\n";
    }
}
