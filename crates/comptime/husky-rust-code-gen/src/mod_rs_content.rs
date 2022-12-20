use crate::*;


pub(crate) fn rust_mod_rs_content(_db: &dyn RustTranspileDb, _module: Term) -> Arc<String> {
    todo!()
    // let mut generator = RustCodeGenerator::new(db, module);
    // let entity_defn = db.entity_defn(module).unwrap();
    // generator.write("use crate::*;\n\n");
    // generator.gen_mod_rs_content(&entity_defn.subentities);
    // Arc::new(generator.finish())
}
