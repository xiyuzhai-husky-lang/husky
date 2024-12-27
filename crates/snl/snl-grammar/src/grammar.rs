use crate::{module::SnlModuleData, rewite_rule::SnlRewriteRuleData, *};
use snl_prelude::path::{module::SnlModulePath, rewrite_rule::SnlRewriteRulePath};
use vec_like::ordered_vec_map::OrderedVecPairMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnlGrammar {
    modules: OrderedVecPairMap<SnlModulePath, SnlModuleData>,
    rewrite_rules: OrderedVecPairMap<SnlRewriteRulePath, SnlRewriteRuleData>,
}

impl SnlGrammar {
    pub(crate) fn new(
        modules: OrderedVecPairMap<SnlModulePath, SnlModuleData>,
        rewrite_rules: OrderedVecPairMap<SnlRewriteRulePath, SnlRewriteRuleData>,
    ) -> Self {
        Self {
            modules,
            rewrite_rules,
        }
    }
}

impl std::ops::Index<SnlModulePath> for SnlGrammar {
    type Output = SnlModuleData;

    fn index(&self, index: SnlModulePath) -> &Self::Output {
        &self.modules[index].1
    }
}

impl std::ops::Index<SnlRewriteRulePath> for SnlGrammar {
    type Output = SnlRewriteRuleData;

    fn index(&self, index: SnlRewriteRulePath) -> &Self::Output {
        &self.rewrite_rules[index].1
    }
}
