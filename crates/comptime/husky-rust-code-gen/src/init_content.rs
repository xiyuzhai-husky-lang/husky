use crate::*;
use code_generator::RustCodeGenerator;

pub(crate) fn rust_init_rs_content(
    db: &dyn RustCodeGenQueryGroup,
    target_entrance: FileItd,
) -> Arc<String> {
    msg_once!("deal with submodules");
    let mut generator = RustCodeGenerator::new_lib(db, target_entrance, true);
    generator.gen_init_content();
    Arc::new(generator.finish())
}
