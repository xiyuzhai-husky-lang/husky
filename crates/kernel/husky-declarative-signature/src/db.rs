use husky_syn_decr::DecrDb;
use husky_syn_expr::ExprRegion;

use crate::*;

pub trait DeclarativeSignatureDb:
    salsa::DbWithJar<DeclarativeSignatureJar> + DecrDb + DeclarativeTermDb
{
    fn declarative_term_region(&self, expr_region: ExprRegion) -> &DeclarativeTermRegion;
}

impl<Db> DeclarativeSignatureDb for Db
where
    Db: salsa::DbWithJar<DeclarativeSignatureJar> + DecrDb + DeclarativeTermDb,
{
    fn declarative_term_region(&self, expr_region: ExprRegion) -> &DeclarativeTermRegion {
        declarative_term_region(self, expr_region)
    }
}
