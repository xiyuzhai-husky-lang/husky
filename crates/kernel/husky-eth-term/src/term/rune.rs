use super::*;

#[salsa::interned(db = EthTermDb, jar = EthTermJar, constructor = new_inner)]
pub struct EthRune {
    pub ty: EthTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub index: RuneIndex,
}

impl EthRune {
    #[inline(always)]
    pub(crate) fn from_dec(db: &::salsa::Db, rune: DecRune) -> EthTermResult<Self> {
        let ty = rune.ty(db)?;
        let ty = EthTerm::ty_from_dec(db, ty)?;
        Ok(Self::new_inner(db, ty, rune.index(db)))
    }

    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        f.write_str("variable_ad_hoc_fmt")
    }
}

impl EthTerm {
    #[track_caller]
    pub fn rune(self) -> EthRune {
        match self {
            EthTerm::Rune(slf) => slf,
            _ => unreachable!(),
        }
    }
}

/// # rewrite

impl EthRune {
    pub fn substitute(self, substitution: EthTermSubstitution, db: &salsa::Db) -> EthTerm {
        if self == substitution.src() {
            return substitution.dst();
        }
        self.substitute_intact(substitution, db).into()
    }

    pub fn substitute_intact(self, substitution: EthTermSubstitution, db: &salsa::Db) -> EthRune {
        Self::new_inner(db, self.ty(db).substitute(substitution, db), self.index(db))
    }
}

impl EthInstantiate for EthRune {
    type Output = Self;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EthInstantiation) -> Self::Output {
        // it's assumed that all symbols will be replaced by its map
        // otherwise it's illegal
        Self::new_inner(
            db,
            self.ty(db).instantiate(db, instantiation),
            self.index(db),
        )
    }
}

/// back to declarative
impl EthRune {
    pub(super) fn into_declarative(self, db: &salsa::Db) -> DecRune {
        DecRune::new(
            Ok(self.ty(db).into_declarative(db)),
            self.index(db).disambiguator(),
            db,
        )
    }
}
