mod utils;

use salsa::Database;

pub(crate) use self::utils::*;

use super::*;

/// representing term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealTermCurry {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_variable: Option<EtherealTermRune>,
    /// X
    pub parameter_ty: EtherealTerm,
    /// Y
    pub return_ty: EtherealTerm,
}

#[test]
fn term_curry_size_works() {
    assert_eq!(
        std::mem::size_of::<EtherealTermCurry>(),
        std::mem::size_of::<u32>()
    );
}

impl EtherealTermCurry {
    pub(crate) fn from_declarative(
        db: &dyn EtherealTermDb,
        declarative_term_curry: DeclarativeTermCurry,
    ) -> EtherealTermResult<Self> {
        term_curry_from_declarative(db, declarative_term_curry)
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        let parameter_variable = self.parameter_variable(db);
        if parameter_variable.is_some() {
            f.write_str("(")?
        }
        f.write_str(self.variance(db).as_str())?;
        if let Some(parameter_variable) = parameter_variable {
            ctx.fmt_with_variable(db, parameter_variable, |ctx| {
                ctx.fmt_variable(db, parameter_variable, f);
                f.write_str(": ")?;
                self.parameter_ty(db).show_with_db_fmt(f, db, ctx)?;
                f.write_str(") -> ")?;
                self.return_ty(db).show_with_db_fmt(f, db, ctx)
            })
        } else {
            self.parameter_ty(db).show_with_db_fmt(f, db, ctx)?;
            f.write_str(" -> ")?;
            self.return_ty(db).show_with_db_fmt(f, db, ctx)
        }
    }

    pub fn substitute(
        self,
        db: &dyn EtherealTermDb,
        substituation: &TermSubstitution,
    ) -> EtherealTerm {
        todo!()
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn term_curry_from_declarative(
    db: &dyn EtherealTermDb,
    declarative_term_curry: DeclarativeTermCurry,
) -> EtherealTermResult<EtherealTermCurry> {
    let t = |declarative_ty| EtherealTerm::ty_from_declarative(db, declarative_ty);
    Ok(EtherealTermCurry::new(
        db,
        declarative_term_curry.curry_kind(db),
        declarative_term_curry.variance(db),
        match declarative_term_curry.parameter_variable(db) {
            Some(parameter_variable) => {
                Some(EtherealTermRune::from_declarative(db, parameter_variable)?)
            }
            None => None,
        },
        t(declarative_term_curry.parameter_ty(db))?,
        t(declarative_term_curry.return_ty(db))?,
    ))
}

impl salsa::DisplayWithDb for EtherealTermCurry {
    fn display_with_db_fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &Db) -> std::fmt::Result {
        self.show_with_db_fmt(f, db(), &mut Default::default())
    }
}
