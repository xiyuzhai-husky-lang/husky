use crate::*;
use code_generator::RustCodeGenerator;
use husky_source_path::SourcePath;

pub(crate) fn rust_registration_rs_content(
    db: &dyn RustTranspileDb,
    target_entrance: SourcePath,
) -> Arc<String> {
    todo!()
    // msg_once!("deal with submodules");
    // let mut generator = RustCodeGenerator::new_lib(db, target_entrance, false);
    // generator.gen_registration_content();
    // Arc::new(generator.finish())
}
