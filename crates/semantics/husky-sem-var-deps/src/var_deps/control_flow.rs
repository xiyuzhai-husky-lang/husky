use super::*;
use control_transfer::SemControlTransferVarDeps;
use domain::SemDomainVarDeps;
use std::ops::Deref;
use value::SemValueVarDeps;

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SemControlFlowVarDeps(OrderedSmallVecSet<SemVarDep, 4>);

impl Deref for SemControlFlowVarDeps {
    type Target = OrderedSmallVecSet<SemVarDep, 4>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SemControlFlowVarDeps {
    pub(crate) fn new(
        value: &SemValueVarDeps,
        control_transfer: &SemControlTransferVarDeps,
        domain: &SemDomainVarDeps,
    ) -> Self {
        Self(
            value
                .iter()
                .chain(control_transfer.iter())
                .chain(domain.iter())
                .copied()
                .collect(),
        )
    }
}
