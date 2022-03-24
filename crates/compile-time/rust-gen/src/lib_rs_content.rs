use crate::*;
use generator::RustGenerator;

pub(crate) fn rust_lib_rs_content(db: &dyn RustGenQueryGroup, main_file: FilePtr) -> Arc<String> {
    let package = db.package(main_file).unwrap();
    msg_once!("deal with submodules");
    let mut generator = RustGenerator::new(db);
    generator.gen_package_lib_rs(&package);
    Arc::new(generator.finish())
}
