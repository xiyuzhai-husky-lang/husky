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
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn DeclarativeTermDb,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        match self.ritchie_kind(db) {
            RitchieKind::FnType => f.write_str("fn(")?,
            RitchieKind::FnTrait => f.write_str("Fn(")?,
            RitchieKind::FnMutTrait => f.write_str("FnMut(")?,
            RitchieKind::GnType => f.write_str("gn(")?,
        }
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

impl<Db> salsa::DisplayWithDb<Db> for DeclarativeTermRitchie
where
    Db: DeclarativeTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclarativeTermJar>>::as_jar_db(db);
        match self.ritchie_kind(db) {
            RitchieKind::FnType => f.write_str("fn(")?,
            RitchieKind::FnTrait => f.write_str("Fn(")?,
            RitchieKind::FnMutTrait => f.write_str("FnMut(")?,
            RitchieKind::GnType => f.write_str("gn(")?,
        }
        for (i, parameter_ty) in self.params(db).iter().enumerate() {
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
#[salsa::debug_with_db(db = DeclarativeTermDb)]
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
        db: &dyn DeclarativeTermDb,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        match self {
            DeclarativeRitchieParameter::Regular(param) => param.show_with_db_fmt(f, db, ctx),
            DeclarativeRitchieParameter::Variadic(param) => param.show_with_db_fmt(f, db, ctx),
            DeclarativeRitchieParameter::Keyed(param) => param.show_with_db_fmt(f, db, ctx),
        }
    }
}

impl<Db> salsa::DisplayWithDb<Db> for DeclarativeRitchieParameter
where
    Db: DeclarativeTermDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        match self {
            DeclarativeRitchieParameter::Regular(parameter) => {
                parameter.display_with_db_fmt(f, db, level)
            }
            DeclarativeRitchieParameter::Variadic(_) => todo!(),
            DeclarativeRitchieParameter::Keyed(_) => todo!(),
        }
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermRitchie {
    fn substitute(
        self,
        _db: &dyn DeclarativeTermDb,
        _substituation: &DeclarativeTermSubstitution,
    ) -> Self {
        todo!()
    }
}
