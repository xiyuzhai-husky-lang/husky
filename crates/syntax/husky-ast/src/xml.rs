use crate::*;
use husky_word::IdentPairMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawXmlExpr {
    pub range: TextRange,
    pub variant: RawXmlExprVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawXmlExprVariant {
    Value(ExprIdx),
    Tag {
        ident: Identifier,
        props: IdentPairMap<ExprIdx>,
    },
}
