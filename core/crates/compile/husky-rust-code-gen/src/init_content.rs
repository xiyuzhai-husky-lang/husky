use crate::*;
use code_generator::Rustcode_generator;

pub(crate) fn rust_init_rs_content(
    db: &dyn RustCodeGenQueryGroup,
    package_main: FilePtr,
) -> Arc<String> {
    msg_once!("deal with submodules");
    let mut generator = Rustcode_generator::new_lib(db, package_main);
    generator.gen_init_content();
    Arc::new(generator.finish())
}
