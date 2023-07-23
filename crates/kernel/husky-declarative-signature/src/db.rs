use husky_syn_decr::DecrDb;
use husky_syn_expr::SynExprRegion;

use crate::*;

pub trait DeclarativeSignatureDb:
    salsa::DbWithJar<DeclarativeSignatureJar> + DecrDb + DeclarativeTermDb
{
    fn declarative_term_region(&self, syn_expr_region: SynExprRegion) -> &DeclarativeTermRegion;
}

impl<Db> DeclarativeSignatureDb for Db
where
    Db: salsa::DbWithJar<DeclarativeSignatureJar> + DecrDb + DeclarativeTermDb,
{
    fn declarative_term_region(&self, syn_expr_region: SynExprRegion) -> &DeclarativeTermRegion {
        declarative_term_region(self, syn_expr_region)
    }
}
