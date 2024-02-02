use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct DeclarativeRitchieRegularParameter {
    contract: TermContract,
    ty: DecTerm,
}

impl DeclarativeRitchieRegularParameter {
    pub fn new(contract: TermContract, ty: DecTerm) -> Self {
        Self { contract, ty }
    }

    pub(crate) fn substitute_ty(self, f: impl FnOnce(DecTerm) -> DecTerm) -> Self {
        Self {
            contract: self.contract,
            ty: f(self.ty),
        }
    }

    pub fn contract(&self) -> TermContract {
        self.contract
    }

    pub fn ty(&self) -> DecTerm {
        self.ty
    }

    #[inline(never)]
    pub(super) fn display_fmt_with_db_and_ctx(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &SymbolDecTermNameMap,
    ) -> std::fmt::Result {
        self.ty.display_fmt_with_db_and_ctx(f, db, ctx)
    }
}
