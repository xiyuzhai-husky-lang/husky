use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CratePathKind {
    Library,
    Binary(Identifier),
}

impl CratePathKind {
    pub fn path(&self) -> &'static str {
        match self {
            CratePathKind::Library => "lib.hsy",
            CratePathKind::Binary(_) => todo!(),
        }
    }
}
