use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GenericPlaceholder {
    Const,
    Type { traits: Vec<RangedEntityRoute> },
}

impl From<&StaticGenericPlaceholder> for GenericPlaceholder {
    fn from(_: &StaticGenericPlaceholder) -> Self {
        todo!()
    }
}
