use crate::*;
use code_generator::RustCodeGenerator;

pub(crate) fn rust_lib_rs_content(
    db: &dyn RustCodeGenQueryGroup,
    target_entrance: FileItd,
) -> Arc<String> {
    todo!()
    // let mut generator = RustCodeGenerator::new_lib(db, target_entrance, true);
    // generator.gen_lib_rs_content();
    // Arc::new(generator.finish())
}
