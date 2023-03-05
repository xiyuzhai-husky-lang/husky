use super::*;

/// representing precise_term `x -> y`
#[salsa::interned(db = PreciseTermDb, jar = PreciseTermJar, constructor = new_inner)]
pub struct PreciseTermRitchie {
    pub ritchie_kind: TermRitchieKind,
    #[return_ref]
    pub parameter_tys: Vec<PreciseTermRitchieParameter>,
    pub return_ty: PreciseTerm,
    // ty: PreciseTerm,
}

impl PreciseTermRitchie {
    pub fn from_raw(
        db: &dyn PreciseTermDb,
        raw_term: RawTermRitchie,
        raw_ty_expectation: TermTypeExpectation,
    ) -> PreciseTermResult<Self> {
        let t = |raw_ty| {
            PreciseTerm::from_raw(db, raw_ty, TermTypeExpectation::FinalDestinationEqsSort)
        };
        Ok(PreciseTermRitchie::new_inner(
            db,
            raw_term.ritchie_kind(db),
            raw_term
                .parameter_tys(db)
                .iter()
                .map(|parameter_ty| {
                    Ok(PreciseTermRitchieParameter {
                        ty: t(parameter_ty.ty())?,
                    })
                })
                .collect::<PreciseTermResult<Vec<_>>>()?,
            t(raw_term.return_ty(db))?,
        ))
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
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

impl PreciseTermRewriteCopy for PreciseTermRitchie {
    fn substitute_copy(
        self,
        db: &dyn PreciseTermDb,
        substituation: &PreciseTermSubstitution,
    ) -> Self {
        todo!()
    }
}
