use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UseAllRule {
    parent: ModulePath,
    // how many symbols have been checked
    progress: usize,
}

impl UseAllRule {
    pub fn new(parent: ModulePath) -> Self {
        Self {
            parent,
            progress: 0,
        }
    }

    pub fn parent(&self) -> ModulePath {
        self.parent
    }

    pub fn progress(&self) -> usize {
        self.progress
    }

    pub(crate) fn is_unresolved(&self, ctx: &EntreeSymbolContext) -> bool {
        self.progress < ctx.module_symbols(self.parent).len()
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub(crate) struct UseAllRules(Vec<UseAllRule>);

pub(crate) struct UseAllRuleIdx(usize);

impl UseAllRules {
    pub(crate) fn indexed_iter(&self) -> impl Iterator<Item = (UseAllRuleIdx, &UseAllRule)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, rule)| (UseAllRuleIdx(i), rule))
    }

    pub(super) fn push(&mut self, new_rule: UseAllRule) {
        self.0.push(new_rule)
    }
}

impl std::ops::Index<UseAllRuleIdx> for UseAllRules {
    type Output = UseAllRule;

    fn index(&self, index: UseAllRuleIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

impl<'a> std::ops::Index<UseAllRuleIdx> for EntreePresheetMut<'a> {
    type Output = UseAllRule;

    fn index(&self, index: UseAllRuleIdx) -> &Self::Output {
        &self.use_all_rules[index]
    }
}
