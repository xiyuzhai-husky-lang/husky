use crate::*;

/// representing declarative_term `X -> Y` or dependent form `(a: X) -> Y(a)`
///
/// refraining from using `new_inner` except in conversion from ethereal term back to declarative term
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar, constructor = pub new_inner)]
pub struct CurryDeclarativeTerm {
    pub toolchain: Toolchain,
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_rune: Option<RuneDeclarativeTerm>,
    /// X
    pub parameter_ty: DeclarativeTerm,
    /// Y
    pub return_ty: DeclarativeTerm,
}

impl CurryDeclarativeTerm {
    /// create a new term curry by converting a symbol to variable
    /// this is the only way to create a new dependent curry declarative term
    /// so that cache hit is maximized
    pub fn new_dependent(
        db: &::salsa::Db,
        toolchain: Toolchain,
        curry_kind: CurryKind,
        variance: Variance,
        // to be converted to variable
        parameter_symbol: SymbolDeclarativeTerm,
        parameter_ty: DeclarativeTerm,
        return_ty: DeclarativeTerm,
    ) -> Self {
        let (return_ty, parameter_rune) = return_ty.create_rune(db, parameter_symbol);
        CurryDeclarativeTerm::new_inner(
            db,
            toolchain,
            curry_kind,
            variance,
            parameter_rune,
            parameter_ty,
            return_ty,
        )
    }

    pub fn new_nondependent(
        db: &::salsa::Db,
        toolchain: Toolchain,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_ty: DeclarativeTerm,
        return_ty: DeclarativeTerm,
    ) -> Self {
        CurryDeclarativeTerm::new_inner(
            db,
            toolchain,
            curry_kind,
            variance,
            None,
            parameter_ty,
            return_ty,
        )
    }

    pub(super) fn substitute_symbol_with_variable(
        self,
        db: &::salsa::Db,
        symbol: SymbolDeclarativeTerm,
        variable: RuneDeclarativeTerm,
    ) -> Self {
        CurryDeclarativeTerm::new_inner(
            db,
            symbol.toolchain(db),
            self.curry_kind(db),
            self.variance(db),
            self.parameter_rune(db),
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
        match self.parameter_rune(db) {
            Some(parameter_rune) => self.return_ty(db).substitute_copy(
                db,
                &DeclarativeTermSubstitution::new(parameter_rune, substitute),
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
        let parameter_rune = self.parameter_rune(db);
        if parameter_rune.is_some() {
            f.write_str("(")?
        }
        f.write_str(self.variance(db).as_str())?;
        if let Some(parameter_rune) = parameter_rune {
            ctx.fmt_with_variable(db, parameter_rune, |ctx| {
                ctx.fmt_variable(db, parameter_rune, f)?;
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
pub(crate) fn curry_parameter_count(db: &::salsa::Db, term: CurryDeclarativeTerm) -> u8 {
    term.return_ty(db).curry_parameter_count(db) + 1
}

impl salsa::DisplayWithDb for CurryDeclarativeTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl DeclarativeTermRewriteCopy for CurryDeclarativeTerm {
    fn substitute_copy(self, db: &::salsa::Db, substitution: &DeclarativeTermSubstitution) -> Self {
        let old_parameter_variable = self.parameter_rune(db);
        let parameter_rune = old_parameter_variable.map(|v| v.substitute_copy(db, substitution));
        let old_parameter_ty = self.parameter_ty(db);
        let parameter_ty = old_parameter_ty.substitute_copy(db, substitution);
        let old_return_ty = self.return_ty(db);
        let return_ty = old_return_ty.substitute_copy(db, substitution);
        if old_parameter_variable == parameter_rune
            && old_parameter_ty == parameter_ty
            && old_return_ty == return_ty
        {
            return self;
        }
        Self::new_inner(
            db,
            self.toolchain(db),
            self.curry_kind(db),
            self.variance(db),
            parameter_rune,
            parameter_ty,
            return_ty,
        )
    }
}
