#[salsa::jar]
pub struct TextJar(crate::line_map::module_text_line_map);
