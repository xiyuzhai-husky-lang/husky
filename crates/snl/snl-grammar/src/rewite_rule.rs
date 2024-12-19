use crate::*;
use snl_prelude::coword::SnlIdent;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SnlRewriteRuleData {
    pub ident: SnlIdent,
    pub description: String,
}
