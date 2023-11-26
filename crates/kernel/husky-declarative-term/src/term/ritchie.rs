mod keyed;
mod regular;
mod variadic;

pub use self::keyed::*;
pub use self::regular::*;
pub use self::variadic::*;

use crate::*;
use smallvec::SmallVec;

/// representing declarative_term `x -> y`
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermRitchie {
    pub ritchie_kind: RitchieKind,
    #[return_ref]
    pub params: SmallVec<[DeclarativeRitchieParameter; 2]>,
    pub return_ty: DeclarativeTerm,
    // ty: DeclarativeTerm,
}

impl DeclarativeTermRitchie {
    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        f.write_str(self.ritchie_kind(db).code())?;
        f.write_str("(")?;
        for (i, parameter_ty) in self.params(db).iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?
            }
            parameter_ty.show_with_db_fmt(f, db, ctx)?
        }
        f.write_str(") -> ")?;
        self.return_ty(db).show_with_db_fmt(f, db, ctx)
    }
}

impl salsa::DisplayWithDb for DeclarativeTermRitchie {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_str(self.ritchie_kind(db).code())?;
        f.write_str("(")?;
        for (i, parameter_ty) in self.params(db).iter().enumerate() {
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
#[salsa::debug_with_db(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
#[enum_class::from_variants]
pub enum DeclarativeRitchieParameter {
    Regular(DeclarativeRitchieRegularParameter),
    Variadic(DeclarativeRitchieVariadicParameter),
    Keyed(DeclarativeRitchieKeyedParameter),
}

impl DeclarativeRitchieParameter {
    pub fn ty(&self) -> DeclarativeTerm {
        match self {
            DeclarativeRitchieParameter::Regular(param) => param.ty(),
            DeclarativeRitchieParameter::Variadic(param) => param.ty(),
            DeclarativeRitchieParameter::Keyed(param) => param.ty(),
        }
    }

    pub(crate) fn substitute_ty(self, f: impl Fn(DeclarativeTerm) -> DeclarativeTerm) -> Self {
        match self {
            DeclarativeRitchieParameter::Regular(param) => param.substitute_ty(f).into(),
            DeclarativeRitchieParameter::Variadic(param) => param.substitute_ty(f).into(),
            DeclarativeRitchieParameter::Keyed(param) => param.substitute_ty(f).into(),
        }
    }

    fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        match self {
            DeclarativeRitchieParameter::Regular(param) => param.show_with_db_fmt(f, db, ctx),
            DeclarativeRitchieParameter::Variadic(param) => param.show_with_db_fmt(f, db, ctx),
            DeclarativeRitchieParameter::Keyed(param) => param.show_with_db_fmt(f, db, ctx),
        }
    }
}

impl salsa::DisplayWithDb for DeclarativeRitchieParameter {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        match self {
            DeclarativeRitchieParameter::Regular(parameter) => parameter.display_with_db_fmt(f, db),
            DeclarativeRitchieParameter::Variadic(_) => todo!(),
            DeclarativeRitchieParameter::Keyed(_) => todo!(),
        }
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermRitchie {
    fn substitute(self, _db: &::salsa::Db, _substituation: &DeclarativeTermSubstitution) -> Self {
        todo!()
    }
}
