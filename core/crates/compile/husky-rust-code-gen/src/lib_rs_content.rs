use crate::*;
use code_generator::Rustcode_generator;

pub(crate) fn rust_lib_rs_content(
    db: &dyn RustCodeGenQueryGroup,
    package_main: FilePtr,
) -> Arc<String> {
    let mut generator = Rustcode_generator::new_lib(db, package_main);
    generator.gen_lib_rs_content();
    Arc::new(generator.finish())
}
