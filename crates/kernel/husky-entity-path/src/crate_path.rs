use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CratePathKind {
    Library,
    Main,
    Binary(Identifier),
}

impl CratePathKind {
    pub fn path(&self) -> &'static str {
        match self {
            CratePathKind::Library => "src/lib.hsy",
            CratePathKind::Main => "src/main.hsy",
            CratePathKind::Binary(_) => todo!(),
        }
    }
}
