use husky_vfs::VfsDb;

pub trait TextDb: salsa::DbWithJar<TextJar> + VfsDb {}

impl<Db> TextDb for Db where Db: salsa::DbWithJar<TextJar> + VfsDb {}

#[salsa::jar(db = TextDb)]
pub struct TextJar(crate::line_map::module_text_line_map);
