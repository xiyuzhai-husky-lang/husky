#[salsa::jar(db = SyntaxFormatDb)]
pub struct SyntaxFormatJar();

pub trait SyntaxFormatDb: salsa::DbWithJar<SyntaxFormatJar> {}

impl<T> SyntaxFormatDb for T where T: salsa::DbWithJar<SyntaxFormatJar> {}
