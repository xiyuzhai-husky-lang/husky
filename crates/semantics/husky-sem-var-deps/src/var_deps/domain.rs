use super::SemVarDep;
use super::*;
use control_flow::SemControlFlowVarDeps;
use control_transfer::SemControlTransferVarDeps;
use value::SemValueVarDeps;
use vec_like::OrderedSmallVecSet;

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SemDomainVarDeps(OrderedSmallVecSet<SemVarDep, 4>);

impl std::ops::Deref for SemDomainVarDeps {
    type Target = OrderedSmallVecSet<SemVarDep, 4>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for &SemDomainVarDeps {
    type Item = SemVarDep;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

impl SemDomainVarDeps {
    pub(crate) fn merge_counted(&mut self, other: &Self, counter: &mut EffectiveMergeCounter) {
        let old_len = self.0.len();
        self.0.extend(&other.0);
        if old_len != self.0.len() {
            counter.count += 1
        }
    }
}

#[derive(Default, Clone)]
pub(crate) struct SemDomainVarDepsGuard {
    deps: OrderedSmallVecSet<SemVarDep, 4>,
}
impl SemDomainVarDepsGuard {
    pub(crate) fn domain_var_deps(&self) -> SemDomainVarDeps {
        SemDomainVarDeps(self.deps.clone())
    }

    pub(crate) fn extend(&mut self, var_deps: &[SemVarDep]) {
        self.deps.extend(var_deps);
    }
}
