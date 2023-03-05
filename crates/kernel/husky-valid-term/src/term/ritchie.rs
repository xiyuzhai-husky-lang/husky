use super::*;
use context::*;

/// representing valid_term `x -> y`
#[salsa::interned(db = ValidTermDb, jar = ValidTermJar)]
pub struct ValidTermRitchie {
    pub ritchie_kind: TermRitchieKind,
    #[return_ref]
    pub parameter_tys: Vec<ValidTermRitchieParameter>,
    pub return_ty: ValidTerm,
    // ty: ValidTerm,
}

impl ValidTermRitchie {
    pub fn from_precise(db: &dyn PreciseTermDb, precise_term: PreciseTermRitchie) -> Self {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
    ) -> std::fmt::Result {
        match self.ritchie_kind(db) {
            TermRitchieKind::Fp => f.write_str("Fp(")?,
            TermRitchieKind::Fn => f.write_str("Fn(")?,
            TermRitchieKind::FnMut => f.write_str("FnMut(")?,
        }
        for (i, parameter_ty) in self.parameter_tys(db).iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?
            }
            parameter_ty.show_with_db_fmt(f, db, ctx)?
        }
        f.write_str(") -> ")?;
        self.return_ty(db).show_with_db_fmt(f, db, ctx)
    }
}

impl<Db> salsa::DisplayWithDb<Db> for ValidTermRitchie
where
    Db: ValidTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<ValidTermJar>>::as_jar_db(db);
        match self.ritchie_kind(db) {
            TermRitchieKind::Fp => f.write_str("Fp(")?,
            TermRitchieKind::Fn => f.write_str("Fn(")?,
            TermRitchieKind::FnMut => f.write_str("FnMut(")?,
        }
        for (i, parameter_ty) in self.parameter_tys(db).iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?
            }
            parameter_ty.display_with_db_fmt(f, db, level.next())?
        }
        f.write_str(") -> ")?;
        self.return_ty(db).display_with_db_fmt(f, db, level.next())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db(db = ValidTermDb)]
pub struct ValidTermRitchieParameter {
    ty: ValidTerm,
}

impl ValidTermRitchieParameter {
    fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
    ) -> std::fmt::Result {
        self.ty.show_with_db_fmt(f, db, ctx)
    }
}

impl<Db> salsa::DisplayWithDb<Db> for ValidTermRitchieParameter
where
    Db: ValidTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<ValidTermJar>>::as_jar_db(db);
        self.ty.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl ValidTermRitchieParameter {
    pub fn new(ty: ValidTerm) -> Self {
        Self { ty }
    }

    pub fn ty(&self) -> ValidTerm {
        self.ty
    }
}

impl ValidTermRewriteCopy for ValidTermRitchie {
    fn substitute_copy(self, db: &dyn ValidTermDb, substituation: &ValidTermSubstitution) -> Self {
        todo!()
    }
}
