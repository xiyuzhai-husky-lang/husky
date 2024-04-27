use super::*;
use husky_eth_term::term::EthTerm;

pub(crate) struct BranchTypeMerger<Expectation: ExpectFlyTerm> {
    /// this is true if the type of one of the branches cannot be inferred
    has_error: bool,
    /// this is true if the type of one of the branches is inferred to be not never
    ever_ty: Option<FlyTerm>,
    expr_expectation: Expectation,
}

impl<Expectation: ExpectFlyTerm> BranchTypeMerger<Expectation> {
    pub(crate) fn new(expr_expectation: Expectation) -> Self {
        Self {
            has_error: false,
            ever_ty: None,
            expr_expectation,
        }
    }

    pub(crate) fn add(&mut self, engine: &SemExprBuilder, new_block_ty: Option<FlyTerm>) {
        match new_block_ty {
            Some(new_block_ty)
                if new_block_ty.base_resolved(engine)
                    == FlyTermBase::Eth(EthTerm::EntityPath(ItemPathTerm::TypeOntology(
                        engine.item_path_menu.never_ty_path(),
                    ))) =>
            {
                ()
            }
            Some(new_block_ty) => {
                if self.ever_ty.is_none() {
                    self.ever_ty = Some(new_block_ty)
                }
            }
            None => self.has_error = true,
        }
    }

    pub(crate) fn merge(self, exhaustive: bool, eth_term_menu: &EthTermMenu) -> Option<FlyTerm> {
        if let Some(ever_ty) = self.ever_ty {
            return ever_ty.into();
        }
        (!self.has_error).then_some(eth_term_menu.never().into())
    }

    pub(crate) fn expr_expectation(&self) -> &Expectation {
        &self.expr_expectation
    }
}
