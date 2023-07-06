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
    pub params: SmallVec<[DeclarativeTermRitchieParameter; 2]>,
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
#[salsa::derive_debug_with_db(db = DeclarativeTermDb)]
#[enum_class::from_variants]
pub enum DeclarativeTermRitchieParameter {
    Regular(DeclarativeTermRitchieRegularParameter),
    Variadic(DeclarativeTermRitchieVariadicParameter),
    Keyed(DeclarativeTermRitchieKeyedParameter),
}

impl DeclarativeTermRitchieParameter {
    pub fn ty(&self) -> DeclarativeTerm {
        match self {
            DeclarativeTermRitchieParameter::Regular(param) => param.ty(),
            DeclarativeTermRitchieParameter::Variadic(_) => todo!(),
            DeclarativeTermRitchieParameter::Keyed(_) => todo!(),
        }
    }

    pub(crate) fn substitute_ty(self, f: impl FnOnce(DeclarativeTerm) -> DeclarativeTerm) -> Self {
        match self {
            DeclarativeTermRitchieParameter::Regular(param) => param.substitute_ty(f).into(),
            DeclarativeTermRitchieParameter::Variadic(_) => todo!(),
            DeclarativeTermRitchieParameter::Keyed(_) => todo!(),
        }
    }

    fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn DeclarativeTermDb,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        match self {
            DeclarativeTermRitchieParameter::Regular(param) => param.show_with_db_fmt(f, db, ctx),
            DeclarativeTermRitchieParameter::Variadic(param) => param.show_with_db_fmt(f, db, ctx),
            DeclarativeTermRitchieParameter::Keyed(param) => param.show_with_db_fmt(f, db, ctx),
        }
    }
}

impl<Db> salsa::DisplayWithDb<Db> for DeclarativeTermRitchieParameter
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
            DeclarativeTermRitchieParameter::Regular(parameter) => {
                parameter.display_with_db_fmt(f, db, level)
            }
            DeclarativeTermRitchieParameter::Variadic(_) => todo!(),
            DeclarativeTermRitchieParameter::Keyed(_) => todo!(),
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
