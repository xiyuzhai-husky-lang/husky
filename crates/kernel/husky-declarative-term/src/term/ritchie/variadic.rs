use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeRitchieVariadicParameter {
    contract: Contract,
    ty: DeclarativeTerm,
}

impl DeclarativeRitchieVariadicParameter {
    pub fn new(contract: Contract, ty: DeclarativeTerm) -> Self {
        Self { contract, ty }
    }

    pub(crate) fn substitute_ty(self, f: impl FnOnce(DeclarativeTerm) -> DeclarativeTerm) -> Self {
        Self {
            contract: self.contract,
            ty: f(self.ty),
        }
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }

    #[inline(never)]
    pub(super) fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn DeclarativeTermDb,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        self.ty.show_with_db_fmt(f, db, ctx)
    }
}

impl salsa::DisplayWithDb for DeclarativeRitchieVariadicParameter {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn Database,
    ) -> std::fmt::Result {
        let db = db.as_jar_db_dyn::<DeclarativeTermJar>();
        f.write_str("...")?;
        self.ty.show_with_db_fmt(f, db, &mut Default::default())
    }
}
