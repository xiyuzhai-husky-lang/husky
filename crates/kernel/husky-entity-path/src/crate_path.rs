use husky_identifier::Identifier;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CratePathKind {
    Library,
    Binary(Identifier),
}
