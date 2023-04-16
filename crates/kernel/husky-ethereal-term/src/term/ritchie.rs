pub use context::*;

use super::*;

/// representing term `x -> y`
#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar, constructor = new_inner)]
pub struct EtherealTermRitchie {
    pub ritchie_kind: TermRitchieKind,
    #[return_ref]
    pub parameter_contracted_tys: Vec<TermRitchieParameterContractedType>,
    pub return_ty: EtherealTerm,
}

#[test]
fn term_ritchie_size_works() {
    assert_eq!(
        std::mem::size_of::<EtherealTermRitchie>(),
        std::mem::size_of::<u32>()
    );
}

impl EtherealTermRitchie {
    //// this constructor guarantees that the result is reduced and first-order valid
    /// returns EtherealTerm instead of EtherealTermApplication because it might reduce to a non application term
    pub fn new(
        db: &dyn EtherealTermDb,
        ritchie_kind: TermRitchieKind,
        parameter_tys: impl IntoIterator<Item = TermRitchieParameterContractedType>,
        return_ty: EtherealTerm,
    ) -> TermResult<EtherealTermRitchie> {
        todo!("check_application_validity(db, function, argument, shift)?");
        Ok(Self::new_unchecked(
            db,
            ritchie_kind,
            parameter_tys,
            return_ty,
        ))
    }

    //// this constructor guarantees that the result is reduced, not necessarily valid
    /// returns EtherealTerm instead of EtherealTermApplication because it might reduce to a non application term
    pub(crate) fn new_unchecked(
        db: &dyn EtherealTermDb,
        ritchie_kind: TermRitchieKind,
        parameter_tys: impl IntoIterator<Item = TermRitchieParameterContractedType>,
        return_ty: EtherealTerm,
    ) -> EtherealTermRitchie {
        Self::new_inner(
            db,
            ritchie_kind,
            parameter_tys
                .into_iter()
                .map(|parameter_contracted_ty| parameter_contracted_ty.reduce(db))
                .collect(),
            return_ty.reduce(db),
        )
    }
    //// this constructor guarantees that the result is reduced, not necessarily valid
    /// returns EtherealTerm instead of EtherealTermApplication because it might reduce to a non application term
    fn new_unchecked2<E>(
        db: &dyn EtherealTermDb,
        ritchie_kind: TermRitchieKind,
        parameter_tys: impl IntoIterator<Item = Result<TermRitchieParameterContractedType, E>>,
        return_ty: EtherealTerm,
    ) -> TermResult<EtherealTermRitchie>
    where
        TermError: From<E>,
    {
        Ok(Self::new_inner(
            db,
            ritchie_kind,
            parameter_tys
                .into_iter()
                .map(|parameter_contracted_ty| Ok(parameter_contracted_ty?.reduce(db)))
                .collect::<TermResult<Vec<_>>>()?,
            return_ty.reduce(db),
        ))
    }

    pub(super) fn reduce(self, db: &dyn EtherealTermDb) -> EtherealTermRitchie {
        todo!()
    }

    #[inline(always)]
    pub(crate) fn from_raw_unchecked(
        db: &dyn EtherealTermDb,
        raw_term_ritchie: DeclarativeTermRitchie,
    ) -> TermResult<Self> {
        term_ritchie_from_raw_unchecked(db, raw_term_ritchie)
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        match self.ritchie_kind(db) {
            TermRitchieKind::FnType => f.write_str("Fp(")?,
            TermRitchieKind::FnTrait => f.write_str("Fn(")?,
            TermRitchieKind::FnMutTrait => f.write_str("FnMut(")?,
        }
        for (i, parameter_contracted_ty) in self.parameter_contracted_tys(db).iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?
            }
            parameter_contracted_ty.show_with_db_fmt(f, db, ctx)?
        }
        f.write_str(") -> ")?;
        self.return_ty(db).show_with_db_fmt(f, db, ctx)
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn term_ritchie_from_raw_unchecked(
    db: &dyn EtherealTermDb,
    raw_term_ritchie: DeclarativeTermRitchie,
) -> TermResult<EtherealTermRitchie> {
    let t = |raw_term| {
        EtherealTerm::from_raw_unchecked(db, raw_term, TermTypeExpectation::FinalDestinationEqsSort)
    };
    EtherealTermRitchie::new_unchecked2(
        db,
        raw_term_ritchie.ritchie_kind(db),
        raw_term_ritchie
            .parameter_tys(db)
            .iter()
            .map(|parameter_contracted_ty| -> TermResult<_> {
                Ok(TermRitchieParameterContractedType {
                    contract: parameter_contracted_ty.contract(),
                    ty: t(parameter_contracted_ty.ty())?,
                })
            }),
        t(raw_term_ritchie.return_ty(db))?,
    )
}

impl<Db> salsa::DisplayWithDb<Db> for EtherealTermRitchie
where
    Db: EtherealTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EtherealTermJar>>::as_jar_db(db);
        match self.ritchie_kind(db) {
            TermRitchieKind::FnType => f.write_str("Fp(")?,
            TermRitchieKind::FnTrait => f.write_str("Fn(")?,
            TermRitchieKind::FnMutTrait => f.write_str("FnMut(")?,
        }
        for (i, parameter_ty) in self.parameter_contracted_tys(db).iter().enumerate() {
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
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub struct TermRitchieParameterContractedType {
    contract: Contract,
    ty: EtherealTerm,
}

impl TermRitchieParameterContractedType {
    fn reduce(self, db: &dyn EtherealTermDb) -> Self {
        Self {
            contract: self.contract,
            ty: self.ty.reduce(db),
        }
    }

    fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        self.ty.show_with_db_fmt(f, db, ctx)
    }
}

impl<Db> salsa::DisplayWithDb<Db> for TermRitchieParameterContractedType
where
    Db: EtherealTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EtherealTermJar>>::as_jar_db(db);
        self.ty.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl TermRitchieParameterContractedType {
    pub fn new(contract: Contract, ty: EtherealTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }
}

impl EtherealTermRitchie {
    fn substitute(self, db: &dyn EtherealTermDb, substituation: &TermSubstitution) -> EtherealTerm {
        todo!()
    }
}
