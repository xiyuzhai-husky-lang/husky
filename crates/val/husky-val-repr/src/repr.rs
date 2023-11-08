use crate::*;
use husky_entity_path::FugitivePath;
use husky_val::{Val, ValDomain, ValOpr};
use smallvec::SmallVec;

#[salsa::interned(db = ValReprDb, jar = ValReprJar)]
pub struct ValRepr {
    pub opr: ValOpr,
    #[return_ref]
    pub opds: SmallVec<[ValRepr; 2]>,
    pub domain: ValDomainRepr,
    pub caching_strategy: ValReprCachingStrategy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValDomainRepr {
    /// everything
    Omni,
    /// those where the val repr of type bool is defined and equals true
    ConditionSatisfied(ValRepr),
    /// those where the val repr of type bool is defined and equals false
    ConditionNotSatisfied(ValRepr),
    /// those where the val repr of type ControlFlow<(), _> is defined and equals Continue(())
    StmtNotReturned(ValRepr),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ValReprCachingStrategy(pub bool);

impl ValRepr {
    pub fn val(self, db: &dyn ValReprDb) -> Val {
        val_repr_val(db, self)
    }

    pub fn expansion(self, db: &dyn ValReprDb) -> ValReprExpansion {
        todo!()
    }
}

#[salsa::tracked(jar = ValReprJar)]
fn val_repr_val(db: &dyn ValReprDb, val_repr: ValRepr) -> Val {
    Val::new(
        db,
        val_repr.opr(db),
        val_repr
            .opds(db)
            .iter()
            .map(|val_repr| val_repr.val(db))
            .collect(),
        val_repr.domain(db).val(db),
    )
}

impl ValDomainRepr {
    pub fn val(self, db: &dyn ValReprDb) -> ValDomain {
        match self {
            ValDomainRepr::Omni => ValDomain::Omni,
            ValDomainRepr::ConditionSatisfied(val_repr) => {
                ValDomain::ConditionSatisfied(val_repr.val(db))
            }
            ValDomainRepr::ConditionNotSatisfied(val_repr) => {
                ValDomain::ConditionNotSatisfied(val_repr.val(db))
            }
            ValDomainRepr::StmtNotReturned(val_repr) => {
                ValDomain::StmtNotReturned(val_repr.val(db))
            }
        }
    }
}
