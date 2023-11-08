use crate::*;
use husky_entity_path::FugitivePath;
use husky_ethereal_term::EtherealTerm;
use husky_hir_defn::HirDefn;
use husky_text_protocol::range::TextRange;
use husky_val::{Val, ValOpr};
use husky_vfs::DiffPath;
use husky_vm::RegularValue;
use smallvec::SmallVec;

#[salsa::interned(db = ValReprDb, jar = ValReprJar)]
pub struct ValRepr {
    pub opr: ValOpr,
    #[return_ref]
    pub opds: SmallVec<[ValRepr; 2]>,
    pub domain: Option<ValRepr>,
    pub caching_strategy: ValReprCachingStrategy,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ValReprCachingStrategy(pub bool);

fn val_repr_val(db: &dyn ValReprDb, val_repr: ValRepr) -> Val {
    Val::new(
        db,
        val_repr.opr(db),
        val_repr
            .opds(db)
            .iter()
            .map(|val_repr| val_repr.val(db))
            .collect(),
        val_repr.domain(db).map(|domain| domain.val(db)),
    )
}

impl ValRepr {
    pub fn val(self, db: &dyn ValReprDb) -> Val {
        val_repr_val(db, self)
    }

    pub fn expansion(self, db: &dyn ValReprDb) -> ValReprExpansion {
        todo!()
    }
}
