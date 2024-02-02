use crate::*;

#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct TraitConstraintDeclarativeTerm {
    ty: DeclarativeTerm,
    trai: DeclarativeTerm,
}

impl TraitConstraintDeclarativeTerm {
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DeclarativeTermRewriteCopy for TraitConstraintDeclarativeTerm {
    fn substitute_copy(
        self,
        _db: &::salsa::Db,
        substitution: &DeclarativeTermSubstitution,
    ) -> Self {
        todo!()
    }
}
