use self::name::DecSvarNameMap;
use crate::*;
use salsa::DisplayWithDb;

impl DecTerm {
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &DecSvarNameMap,
    ) -> std::fmt::Result {
        match self {
            DecTerm::Literal(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
            DecTerm::Symbol(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
            DecTerm::Hvar(term) => term.display_fmt_with_db(f, db),
            DecTerm::EntityPath(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
            DecTerm::Category(term) => f.write_str(&term.to_string()),
            DecTerm::Universe(term) => f.write_str(&term.to_string()),
            DecTerm::Curry(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
            DecTerm::Ritchie(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
            DecTerm::Abstraction(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
            DecTerm::Application(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
            DecTerm::ApplicationOrRitchieCall(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
            DecTerm::AssocItem(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
            DecTerm::TypeAsTraitItem(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
            DecTerm::TraitConstraint(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
            DecTerm::LeashOrBitNot(_) => f.write_str("~"),
            DecTerm::List(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
            DecTerm::Wrapper(term) => term.display_fmt_with_db_and_ctx(f, db, ctx),
        }
    }
}
