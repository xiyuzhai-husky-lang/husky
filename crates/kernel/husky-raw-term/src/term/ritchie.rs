pub use context::*;

use crate::*;

/// representing raw_term `x -> y`
#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermRitchie {
    pub ritchie_kind: TermRitchieKind,
    #[return_ref]
    pub parameter_tys: Vec<RawTermRitchieParameterLiasonedType>,
    pub return_ty: RawTerm,
    // ty: RawTerm,
}

impl RawTermRitchie {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        match self.ritchie_kind(db) {
            TermRitchieKind::FnType => f.write_str("Fp(")?,
            TermRitchieKind::FnTrait => f.write_str("Fn(")?,
            TermRitchieKind::FnMutTrait => f.write_str("FnMut(")?,
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

impl<Db> salsa::DisplayWithDb<Db> for RawTermRitchie
where
    Db: RawTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        match self.ritchie_kind(db) {
            TermRitchieKind::FnType => f.write_str("Fp(")?,
            TermRitchieKind::FnTrait => f.write_str("Fn(")?,
            TermRitchieKind::FnMutTrait => f.write_str("FnMut(")?,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = RawTermDb)]
pub struct RawTermRitchieParameterLiasonedType {
    liason: Liason,
    ty: RawTerm,
}

impl RawTermRitchieParameterLiasonedType {
    pub fn new(liason: Liason, ty: RawTerm) -> Self {
        Self { liason, ty }
    }

    pub(crate) fn substitute_ty(self, f: impl FnOnce(RawTerm) -> RawTerm) -> Self {
        Self {
            liason: self.liason,
            ty: f(self.ty),
        }
    }

    pub fn liason(&self) -> Liason {
        self.liason
    }

    pub fn ty(&self) -> RawTerm {
        self.ty
    }
}

impl RawTermRitchieParameterLiasonedType {
    fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        self.ty.show_with_db_fmt(f, db, ctx)
    }
}

impl<Db> salsa::DisplayWithDb<Db> for RawTermRitchieParameterLiasonedType
where
    Db: RawTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        self.ty.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl RawTermRewriteCopy for RawTermRitchie {
    fn substitute(self, _db: &dyn RawTermDb, _substituation: &RawTermSubstitution) -> Self {
        todo!()
    }
}
