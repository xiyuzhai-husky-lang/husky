use super::*;
use husky_declarative_term::term::abstraction::AbstractionDeclarativeTerm;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct AbstractionEtherealTerm {
    x: RuneEtherealTerm,
    m: EtherealTerm,
}

#[test]
fn term_abstraction_size_works() {
    assert_eq!(
        std::mem::size_of::<AbstractionEtherealTerm>(),
        std::mem::size_of::<u32>()
    );
}

impl AbstractionEtherealTerm {
    pub(crate) fn from_declarative(
        _db: &::salsa::Db,
        _precise_term: AbstractionDeclarativeTerm,
        _term_ty_expectation: TermTypeExpectation,
    ) -> EtherealTermResult<Self> {
        todo!()
    }

    pub fn ty(&self) -> EtherealTerm {
        todo!()
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

/// # rewrite

impl AbstractionEtherealTerm {
    pub fn substitute(self, substitution: EtherealTermSubstitution, db: &::salsa::Db) -> Self {
        let old_x = self.x(db);
        let new_x = old_x.substitute_intact(substitution, db);
        let old_m = self.m(db);
        let new_m = old_m.substitute(substitution, db);
        if old_x == new_x && old_m == new_m {
            return self;
        }
        Self::new(db, new_x, new_m)
    }
}

impl EtherealTermInstantiate for AbstractionEtherealTerm {
    type Output = AbstractionEtherealTerm;

    fn instantiate(self, db: &salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        Self::new(
            db,
            self.x(db).instantiate(db, instantiation),
            self.m(db).instantiate(db, instantiation),
        )
    }
}

impl salsa::DisplayWithDb for AbstractionEtherealTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        // use std::fmt::Write;
        // f.write_char(husky_unicode_symbols::greek::GREEK_LETTER_LOWERCASE_LAMBDA);
        // todo!()
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}
