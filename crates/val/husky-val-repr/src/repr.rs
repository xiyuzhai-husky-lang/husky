use crate::*;
use husky_entity_path::FugitivePath;
use husky_val::{Val, ValDomain, ValOpr};
use smallvec::{smallvec, SmallVec};

#[salsa::interned(db = ValReprDb, jar = ValReprJar)]
pub struct ValRepr {
    pub opr: ValOpr,
    #[return_ref]
    pub opds: SmallVec<[ValRepr; 2]>,
    pub domain: ValDomainRepr,
    pub caching_strategy: ValReprCachingStrategy,
}

impl ValRepr {
    pub(crate) fn new_val_item(path: FugitivePath, db: &dyn ValReprDb) -> Self {
        let opr = ValOpr::Fugitive(path);
        let opds = smallvec![];
        let domain = ValDomainRepr::Omni;
        let caching_strategy = ValReprCachingStrategy::Cache;
        Self::new(db, opr, opds, domain, caching_strategy)
    }
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
pub enum ValReprCachingStrategy {
    Cache,
    Skip,
}

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

#[cfg(test)]
fn val_item_val_reprs(db: &DB, module_path: ModulePath) -> Vec<ValRepr> {
    use husky_entity_kind::FugitiveKind;
    use husky_entity_path::{ItemPath, MajorItemPath};
    use husky_entity_syn_tree::helpers::paths::module_item_paths;
    use husky_hir_defn::HasHirDefn;

    module_item_paths(db, module_path)
        .as_ref()
        .expect("all modules should be guaranteed to be valid")
        .iter()
        .filter_map(|&path| match path {
            ItemPath::MajorItem(MajorItemPath::Fugitive(path)) => match path.fugitive_kind(db) {
                FugitiveKind::Val => Some(ValRepr::new_val_item(path, db)),
                _ => None,
            },
            _ => None,
        })
        .collect()
}
