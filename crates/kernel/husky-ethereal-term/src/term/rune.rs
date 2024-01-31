use super::*;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar, constructor = new_inner)]
pub struct RuneEtherealTerm {
    pub ty: EtherealTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: RuneIndex,
}

impl RuneEtherealTerm {
    #[inline(always)]
    pub(crate) fn from_declarative(
        db: &::salsa::Db,
        variable: RuneDeclarativeTerm,
    ) -> EtherealTermResult<Self> {
        let ty = variable.ty(db)?;
        let ty = EtherealTerm::ty_from_declarative(db, ty)?;
        Ok(Self::new_inner(db, ty, variable.idx(db)))
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_variable(db, self, f)
    }
}

impl EtherealTerm {
    #[track_caller]
    pub fn rune(self) -> RuneEtherealTerm {
        match self {
            EtherealTerm::Rune(slf) => slf,
            _ => unreachable!(),
        }
    }
}

/// # rewrite
impl RuneEtherealTerm {
    pub fn substitute(
        self,
        substitution: EtherealTermSubstitution,
        db: &salsa::Db,
    ) -> EtherealTerm {
        if self == substitution.src() {
            return substitution.dst();
        }
        self.substitute_intact(substitution, db).into()
    }

    pub fn substitute_intact(
        self,
        substitution: EtherealTermSubstitution,
        db: &salsa::Db,
    ) -> RuneEtherealTerm {
        Self::new_inner(db, self.ty(db).substitute(substitution, db), self.idx(db))
    }
}

/// back to declarative
impl RuneEtherealTerm {
    pub(super) fn into_declarative(self, db: &salsa::Db) -> RuneDeclarativeTerm {
        RuneDeclarativeTerm::new(
            Ok(self.ty(db).into_declarative(db)),
            self.idx(db).disambiguator(),
            db,
        )
    }
}
