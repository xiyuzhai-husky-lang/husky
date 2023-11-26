pub use context::*;

use crate::*;

/// representing declarative_term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar, constructor = new_inner)]
pub struct DeclarativeTermCurry {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_variable: Option<DeclarativeTermRune>,
    /// X
    pub parameter_ty: DeclarativeTerm,
    /// Y
    pub return_ty: DeclarativeTerm,
}

impl DeclarativeTermCurry {
    /// create a new term curry through converting a symbol to variable
    /// this is the only to crate a new term curry
    /// so that cache hit is maximized
    pub fn new_dependent(
        db: &::salsa::Db,
        curry_kind: CurryKind,
        variance: Variance,
        // to be converted to variable
        parameter_symbol: DeclarativeTermSymbol,
        parameter_ty: DeclarativeTerm,
        return_ty: DeclarativeTerm,
    ) -> Self {
        let (return_ty, parameter_variable) = return_ty.r#abstract(db, parameter_symbol);
        DeclarativeTermCurry::new_inner(
            db,
            curry_kind,
            variance,
            parameter_variable,
            parameter_ty,
            return_ty,
        )
    }

    pub fn new_nondependent(
        db: &::salsa::Db,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_ty: DeclarativeTerm,
        return_ty: DeclarativeTerm,
    ) -> Self {
        DeclarativeTermCurry::new_inner(db, curry_kind, variance, None, parameter_ty, return_ty)
    }

    pub(super) fn substitute_symbol_with_variable(
        self,
        db: &::salsa::Db,
        symbol: DeclarativeTermSymbol,
        variable: DeclarativeTermRune,
    ) -> Self {
        DeclarativeTermCurry::new_inner(
            db,
            self.curry_kind(db),
            self.variance(db),
            self.parameter_variable(db),
            self.parameter_ty(db)
                .substitute_symbol_with_variable(db, symbol, variable),
            self.return_ty(db)
                .substitute_symbol_with_variable(db, symbol, variable),
        )
    }

    pub fn return_ty_with_variable_substituted(
        self,
        db: &::salsa::Db,
        substitute: DeclarativeTerm,
    ) -> DeclarativeTerm {
        match self.parameter_variable(db) {
            Some(parameter_variable) => self.return_ty(db).substitute(
                db,
                &DeclarativeTermSubstitution::new(parameter_variable, substitute),
            ),
            None => self.return_ty(db),
        }
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut DeclarativeTermShowContext,
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
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn curry_parameter_count(db: &::salsa::Db, term: DeclarativeTermCurry) -> u8 {
    term.return_ty(db).curry_parameter_count(db) + 1
}

impl salsa::DisplayWithDb for DeclarativeTermCurry {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        let db = db();
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermCurry {
    fn substitute(self, db: &::salsa::Db, substituation: &DeclarativeTermSubstitution) -> Self {
        let old_parameter_variable = self.parameter_variable(db);
        let parameter_variable = old_parameter_variable.map(|v| v.substitute(db, substituation));
        let old_parameter_ty = self.parameter_ty(db);
        let parameter_ty = old_parameter_ty.substitute(db, substituation);
        let old_return_ty = self.return_ty(db);
        let return_ty = old_return_ty.substitute(db, substituation);
        if old_parameter_variable == parameter_variable
            && old_parameter_ty == parameter_ty
            && old_return_ty == return_ty
        {
            return self;
        }
        Self::new_inner(
            db,
            self.curry_kind(db),
            self.variance(db),
            parameter_variable,
            parameter_ty,
            return_ty,
        )
    }
}
