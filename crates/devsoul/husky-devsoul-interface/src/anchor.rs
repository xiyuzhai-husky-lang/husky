#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor<StaticVarId> {
    Specific(StaticVarId),
    Generic { limit: usize },
}
