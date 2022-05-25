#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum StructItemContext {
    OriginalField = 0,
    DerivedField = 1,
    AssociatedCall = 2,
    Method = 3,
    TraitImpl = 4,
    Visual = 5,
}
