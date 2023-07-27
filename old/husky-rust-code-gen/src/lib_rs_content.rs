use crate::*;

pub(crate) fn rust_lib_rs_content(
    _db: &dyn RustTranspileDb,
    _target_entrance: DiffPath,
) -> Arc<String> {
    todo!()
    // let mut generator = RustCodeGenerator::new_lib(db, target_entrance, true);
    // generator.gen_lib_rs_content();
    // Arc::new(generator.finish())
}
