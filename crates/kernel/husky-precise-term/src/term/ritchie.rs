pub use context::*;

use crate::*;

/// representing precise_term `x -> y`
#[salsa::interned(db = PreciseTermDb, jar = PreciseTermJar)]
pub struct PreciseTermRitchie {
    pub ritchie_kind: PreciseTermRitchieKind,
    #[return_ref]
    pub parameter_tys: Vec<PreciseTermRitchieParameter>,
    pub return_ty: PreciseTerm,
    // ty: PreciseTerm,
}

impl PreciseTermRitchie {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        match self.ritchie_kind(db) {
            PreciseTermRitchieKind::Fp => f.write_str("Fp(")?,
            PreciseTermRitchieKind::Fn => f.write_str("Fn(")?,
            PreciseTermRitchieKind::FnMut => f.write_str("FnMut(")?,
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

impl<Db> salsa::DisplayWithDb<Db> for PreciseTermRitchie
where
    Db: PreciseTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<PreciseTermJar>>::as_jar_db(db);
        match self.ritchie_kind(db) {
            PreciseTermRitchieKind::Fp => f.write_str("Fp(")?,
            PreciseTermRitchieKind::Fn => f.write_str("Fn(")?,
            PreciseTermRitchieKind::FnMut => f.write_str("FnMut(")?,
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
#[salsa::derive_debug_with_db(db = PreciseTermDb)]
pub struct PreciseTermRitchieParameter {
    ty: PreciseTerm,
}

impl PreciseTermRitchieParameter {
    fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        self.ty.show_with_db_fmt(f, db, ctx)
    }
}

impl<Db> salsa::DisplayWithDb<Db> for PreciseTermRitchieParameter
where
    Db: PreciseTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<PreciseTermJar>>::as_jar_db(db);
        self.ty.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl PreciseTermRitchieParameter {
    pub fn new(ty: PreciseTerm) -> Self {
        Self { ty }
    }

    pub fn ty(&self) -> PreciseTerm {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PreciseTermRitchieKind {
    Fp,
    Fn,
    FnMut,
}

impl PreciseTermRewriteCopy for PreciseTermRitchie {
    fn substitute_copy(
        self,
        db: &dyn PreciseTermDb,
        substituation: &PreciseTermSubstitution,
    ) -> Self {
        todo!()
    }
}
