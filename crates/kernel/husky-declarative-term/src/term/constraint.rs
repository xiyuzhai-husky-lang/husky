use crate::*;

#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermTraitConstraint {
    ty: DeclarativeTerm,
    trai: DeclarativeTerm,
}

impl DeclarativeTermTraitConstraint {
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermTraitConstraint {
    fn substitute(self, _db: &::salsa::Db, _substituation: &DeclarativeTermSubstitution) -> Self {
        todo!()
    }
}
