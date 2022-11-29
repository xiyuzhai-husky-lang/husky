use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CratePathKind {
    Library,
    Binary(Identifier),
}
