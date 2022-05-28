#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum StructItemContext {
    OriginalField = 0,
    DefaultField = 1,
    DerivedEagerField = 2,
    DerivedLazyField = 3,
    AssociatedCall = 4,
    Method = 5,
    TraitImpl = 6,
    Visual = 7,
}
