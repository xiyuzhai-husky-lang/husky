use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct DeclarativeRitchieVariadicParameter {
    contract: Contract,
    ty: DecTerm,
}

impl DeclarativeRitchieVariadicParameter {
    pub fn new(contract: Contract, ty: DecTerm) -> Self {
        Self { contract, ty }
    }

    pub(crate) fn substitute_ty(self, f: impl FnOnce(DecTerm) -> DecTerm) -> Self {
        Self {
            contract: self.contract,
            ty: f(self.ty),
        }
    }

    pub fn contract(&self) -> Contract {
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
        ctx: &DecSvarNameMap,
    ) -> std::fmt::Result {
        self.ty.display_fmt_with_db_and_ctx(f, db, ctx)
    }
}
