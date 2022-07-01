use crate::*;
use generator::RustCodeGenerator;

pub(crate) fn rust_mod_rs_content(
    db: &dyn RustCodeGenQueryGroup,
    module: EntityRoutePtr,
) -> Arc<String> {
    let mut generator = RustCodeGenerator::new(db, module);
    let entity_defn = db.entity_defn(module).unwrap();
    generator.write("use crate::*;\n\n");
    generator.gen_mod_rs_content(&entity_defn.subentities);
    Arc::new(generator.finish())
}
