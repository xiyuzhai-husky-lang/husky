#[salsa::jar]
pub struct CodeLensJar(
    crate::code_lens::module_code_lenses,
    crate::code_lens::item_code_lenses_data,
);
