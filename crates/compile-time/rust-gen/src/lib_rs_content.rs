use crate::*;
use generator::RustGenerator;

pub(crate) fn rust_lib_rs_content(db: &dyn RustGenQueryGroup, main_file: FilePtr) -> Arc<String> {
    let pack = db.package(main_file).unwrap();
    emsg_once!("deal with submodules");
    let mut generator = RustGenerator::new(db);
    generator.gen_pack_lib_rs(&pack);
    Arc::new(generator.finish())
}
