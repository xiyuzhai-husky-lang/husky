use crate::*;
use word::IdentPairDict;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawXmlExpr {
    pub ident: CustomIdentifier,
    pub props: IdentPairDict<RawExprIdx>,
    pub range: TextRange,
}
