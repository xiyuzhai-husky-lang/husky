use crate::*;


pub(crate) fn rust_registration_rs_content(
    _db: &dyn RustTranspileDb,
    _target_entrance: AbsolutePath,
) -> Arc<String> {
    todo!()
    // msg_once!("deal with submodules");
    // let mut generator = RustCodeGenerator::new_lib(db, target_entrance, false);
    // generator.gen_registration_content();
    // Arc::new(generator.finish())
}
