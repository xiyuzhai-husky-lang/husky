use crate::*;
use code_generator::RustCodeGenerator;

pub(crate) fn rust_init_rs_content(
    db: &dyn RustCodeGenQueryGroup,
    package_main: FilePtr,
) -> Arc<String> {
    emsg_once!("deal with submodules");
    let mut generator = RustCodeGenerator::new_lib(db, package_main);
    generator.gen_init_content();
    Arc::new(generator.finish())
}
