mod keyed;
mod regular;
mod variadic;

use salsa::Db;

pub use self::keyed::*;
pub use self::regular::*;
pub use self::variadic::*;

use super::*;

/// representing term `x -> y`
#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar, constructor = new_inner)]
pub struct EtherealTermRitchie {
    pub ritchie_kind: RitchieKind,
    #[return_ref]
    pub parameter_contracted_tys: Vec<EtherealRitchieParameter>,
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
        db: &::salsa::Db,
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: impl IntoIterator<Item = EtherealRitchieParameter>,
        return_ty: EtherealTerm,
    ) -> EtherealTermResult<EtherealTermRitchie> {
        // todo!("check_application_validity(db, function, argument, shift)?");
        Ok(Self::new_unchecked(
            db,
            ritchie_kind,
            parameter_contracted_tys,
            return_ty,
        ))
    }

    /// this constructor guarantees that the result is reduced, not necessarily valid;
    ///
    /// returns EtherealTerm instead of EtherealTermApplication because it might reduce to a non application term
    pub(crate) fn new_unchecked(
        db: &::salsa::Db,
        ritchie_kind: RitchieKind,
        parameter_tys: impl IntoIterator<Item = EtherealRitchieParameter>,
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
    /// this constructor guarantees that the result is reduced, not necessarily valid
    ///
    /// returns EtherealTerm instead of EtherealTermApplication because it might reduce to a non application term
    fn new_unchecked2<E>(
        db: &::salsa::Db,
        ritchie_kind: RitchieKind,
        parameter_tys: impl IntoIterator<Item = Result<EtherealRitchieParameter, E>>,
        return_ty: EtherealTerm,
    ) -> EtherealTermResult<EtherealTermRitchie>
    where
        EtherealTermError: From<E>,
    {
        Ok(Self::new_inner(
            db,
            ritchie_kind,
            parameter_tys
                .into_iter()
                .map(|parameter_contracted_ty| Ok(parameter_contracted_ty?.reduce(db)))
                .collect::<EtherealTermResult<Vec<_>>>()?,
            return_ty.reduce(db),
        ))
    }

    pub(super) fn reduce(self, db: &::salsa::Db) -> EtherealTermRitchie {
        // ad hoc
        self
    }

    #[inline(always)]
    pub(crate) fn from_declarative(
        db: &::salsa::Db,
        declarative_term_ritchie: DeclarativeTermRitchie,
    ) -> EtherealTermResult<Self> {
        ethereal_term_ritchie_from_declarative_term_ritchie(db, declarative_term_ritchie)
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        f.write_str(self.ritchie_kind(db).code())?;
        f.write_str("(")?;
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
pub(crate) fn ethereal_term_ritchie_from_declarative_term_ritchie(
    db: &::salsa::Db,
    declarative_term_ritchie: DeclarativeTermRitchie,
) -> EtherealTermResult<EtherealTermRitchie> {
    EtherealTermRitchie::new_unchecked2(
        db,
        declarative_term_ritchie.ritchie_kind(db),
        declarative_term_ritchie
            .params(db)
            .iter()
            .map(|&param| -> EtherealTermResult<_> {
                EtherealRitchieParameter::from_declarative(db, param)
            }),
        EtherealTerm::ty_from_declarative(db, declarative_term_ritchie.return_ty(db))?,
    )
}

impl EtherealRitchieParameter {
    pub fn from_declarative(
        db: &::salsa::Db,
        param: DeclarativeRitchieParameter,
    ) -> EtherealTermResult<Self> {
        Ok(match param {
            DeclarativeRitchieParameter::Regular(param) => {
                EtherealRitchieRegularParameter::from_declarative(db, param)?.into()
            }
            DeclarativeRitchieParameter::Variadic(param) => {
                EtherealRitchieVariadicParameter::from_declarative(db, param)?.into()
            }
            DeclarativeRitchieParameter::Keyed(param) => {
                EtherealRitchieKeyedParameter::from_declarative(db, param)?.into()
            }
        })
    }
}

impl EtherealInstantiate for EtherealRitchieParameter {
    type Output = Self;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        todo!()
    }
}

impl salsa::DisplayWithDb for EtherealTermRitchie {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_str(self.ritchie_kind(db).code())?;
        f.write_str("(")?;
        for (i, parameter_ty) in self.parameter_contracted_tys(db).iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?
            }
            parameter_ty.display_with_db_fmt(f, db)?
        }
        f.write_str(") -> ")?;
        self.return_ty(db).display_with_db_fmt(f, db)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EtherealTermDb, jar = EtherealTermJar)]
#[enum_class::from_variants]
pub enum EtherealRitchieParameter {
    Regular(EtherealRitchieRegularParameter),
    Variadic(EtherealRitchieVariadicParameter),
    Keyed(EtherealRitchieKeyedParameter),
}

impl EtherealRitchieParameter {
    fn reduce(self, db: &::salsa::Db) -> Self {
        match self {
            EtherealRitchieParameter::Regular(param) => param.reduce(db).into(),
            EtherealRitchieParameter::Variadic(param) => param.reduce(db).into(),
            EtherealRitchieParameter::Keyed(param) => param.reduce(db).into(),
        }
    }

    fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        match self {
            EtherealRitchieParameter::Regular(param) => param.show_with_db_fmt(f, db, ctx),
            EtherealRitchieParameter::Variadic(param) => param.show_with_db_fmt(f, db, ctx),
            EtherealRitchieParameter::Keyed(param) => param.show_with_db_fmt(f, db, ctx),
        }
    }
}

impl salsa::DisplayWithDb for EtherealRitchieParameter {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        match self {
            EtherealRitchieParameter::Regular(param) => param.display_with_db_fmt(f, db),
            EtherealRitchieParameter::Variadic(param) => param.display_with_db_fmt(f, db),
            EtherealRitchieParameter::Keyed(param) => param.display_with_db_fmt(f, db),
        }
    }
}

impl EtherealRitchieParameter {
    // pub fn new(contract: Contract, ty: EtherealTerm) -> Self {
    //     Self { contract, ty }
    // }

    // pub fn contract(&self) -> Contract {
    //     self.contract
    // }

    pub fn ty(&self) -> EtherealTerm {
        match self {
            EtherealRitchieParameter::Regular(param) => param.ty(),
            EtherealRitchieParameter::Variadic(param) => param.ty(),
            EtherealRitchieParameter::Keyed(param) => param.ty(),
        }
    }
}

impl EtherealTermRitchie {
    fn substitute(self, db: &::salsa::Db, substituation: &TermSubstitution) -> EtherealTerm {
        todo!()
    }
}
