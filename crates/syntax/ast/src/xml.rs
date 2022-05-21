use crate::*;
use word::IdentPairDict;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawXmlExpr {
    pub props: IdentPairDict<RawExprIdx>,
}
