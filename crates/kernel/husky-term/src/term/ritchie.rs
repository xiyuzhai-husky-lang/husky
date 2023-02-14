pub use context::*;

use crate::*;

/// representing term `x -> y`
#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermRitchie {
    pub ritchie_kind: TermRitchieKind,
    #[return_ref]
    pub parameter_tys: Vec<TermRitchieParameter>,
    pub return_ty: Term,
    // ty: Term,
}

impl<Db> salsa::DisplayWithDb<Db> for TermRitchie
where
    Db: TermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        match self.ritchie_kind(db) {
            TermRitchieKind::Fp => f.write_str("Fp(")?,
            TermRitchieKind::Fn => f.write_str("Fn(")?,
            TermRitchieKind::FnMut => f.write_str("FnMut(")?,
        }
        for (i, parameter_ty) in self.parameter_tys(db).iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?
            }
            parameter_ty.display_with_db_fmt(f, db, include_all_fields)?
        }
        f.write_str(") -> ")?;
        self.return_ty(db)
            .display_with_db_fmt(f, db, include_all_fields)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TermRitchieParameter {
    ty: Term,
}

impl<Db> salsa::DisplayWithDb<Db> for TermRitchieParameter
where
    Db: TermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.ty.display_with_db_fmt(f, db, include_all_fields)
    }
}

impl TermRitchieParameter {
    pub fn new(ty: Term) -> Self {
        Self { ty }
    }

    pub fn ty(&self) -> Term {
        self.ty
    }
}

impl<Db: TermDb + ?Sized> salsa::DebugWithDb<Db> for TermRitchieParameter {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        f.debug_struct("TermRitchieParameter")
            .field("ty", &self.ty.debug_with(db, include_all_fields))
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermRitchieKind {
    Fp,
    Fn,
    FnMut,
}

impl TermRewriteCopy for TermRitchie {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        todo!()
    }
}
