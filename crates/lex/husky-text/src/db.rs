#[salsa::jar(db = TextDb)]
pub struct TextJar(crate::line_map::module_text_line_map);
