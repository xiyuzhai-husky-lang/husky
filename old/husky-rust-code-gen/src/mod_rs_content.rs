use crate::*;

pub(crate) fn rust_mod_rs_content(_db: &dyn RustTranspileDb, _module: EtherealTerm) -> Arc<String> {
    todo!()
    // let mut generator = RustCodeGenerator::new(db, module);
    // let item_defn = db.item_defn(module).unwrap();
    // generator.write("use crate::*;\n\n");
    // generator.gen_mod_rs_content(&item_defn.subentities);
    // Arc::new(generator.finish())
}
