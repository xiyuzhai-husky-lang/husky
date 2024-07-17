use crate::{signature::HasEthSignature, EthSignatureResult};
use husky_entity_path::path::ItemPath;
use husky_entity_tree::region_path::SynNodeRegionPath;
use husky_eth_term::{
    context::EthTermContextItd, instantiation::IsEthTermContextRef, term::EthTerm, EthTermError,
    EthTermResult,
};
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EthTermContextRef {
    context_itd: EthTermContextItd,
    task_ty: Option<EthTerm>,
}

impl EthTermContextRef {
    fn new_generic(db: &::salsa::Db) -> Self {
        Self::from_context_itd(EthTermContextItd::new_generic(db), db)
    }

    pub fn new(region_path: SynNodeRegionPath, db: &::salsa::Db) -> EthSignatureResult<Self> {
        let package_path = region_path.package_path(db);
        let task_ty = package_path.eth_signature(db)?.data(db).task_ty();
        let context_itd = EthTermContextItd::new(db, task_ty);
        Ok(Self {
            task_ty,
            context_itd,
        })
    }

    pub(crate) fn from_context_itd(context_itd: EthTermContextItd, db: &::salsa::Db) -> Self {
        Self {
            context_itd,
            task_ty: context_itd.task_ty(db),
        }
    }
}

pub(crate) trait HasEthTermContextRef: Copy {
    fn context_ref(self, db: &::salsa::Db) -> EthTermContextRef;
}

impl HasEthTermContextRef for EthTermContextItd {
    fn context_ref(self, db: &salsa::Db) -> EthTermContextRef {
        EthTermContextRef::from_context_itd(self, db)
    }
}

impl EthTermContextRef {
    pub fn task_ty(&self) -> Option<EthTerm> {
        self.task_ty
    }
}

impl<'db> IsEthTermContextRef<'db> for EthTermContextRef {
    fn reduce_ty_as_trai_item(
        &self,
        term: husky_eth_term::term::trai_for_ty_item::EthTypeAsTraitItem,
    ) -> EthTerm {
        todo!()
    }

    fn task_ty(&self) -> Option<EthTerm> {
        self.task_ty
    }

    fn context_itd(&self) -> husky_eth_term::context::EthTermContextItd {
        self.context_itd
    }
}
