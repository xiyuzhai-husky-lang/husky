use crate::*;

pub(crate) fn rust_init_rs_content(
    _db: &dyn RustTranspileDb,
    _target_entrance: DiffPath,
) -> Arc<String> {
    todo!()
    // msg_once!("deal with submodules");
    // let mut generator = RustCodeGenerator::new_lib(db, target_entrance, true);
    // generator.gen_init_content();
    // Arc::new(generator.finish())
}
