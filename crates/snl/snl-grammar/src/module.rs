use crate::{rewite_rule::SnlRewriteRuleData, *};
use snl_prelude::{coword::SnlIdentMap, path::module::SnlModulePath};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SnlModuleData {
    Nonterminal(SnlIdentMap<SnlModulePath>),
    Terminal(SnlIdentMap<SnlRewriteRuleData>),
}
