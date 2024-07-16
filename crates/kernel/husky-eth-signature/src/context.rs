use husky_entity_path::path::ItemPath;
use husky_entity_tree::region_path::SynNodeRegionPath;
use husky_eth_term::{
    instantiation::IsEthInstantiationContext, term::EthTerm, EthTermError, EthTermResult,
};

use crate::{signature::HasEthSignature, EthSignatureResult};

#[salsa::interned(constructor = new_inner)]
pub struct EthSignatureBuilderContextItd {
    #[return_ref]
    pub context: EthSignatureBuilderContext,
}

impl EthSignatureBuilderContextItd {
    #[deprecated(note = "we should probably use a better notion")]
    pub fn new_generic(db: &::salsa::Db) -> Self {
        let context = EthSignatureBuilderContext::new_generic();
        Self::new_inner(db, context)
    }

    pub fn new(region_path: SynNodeRegionPath, db: &::salsa::Db) -> EthSignatureResult<Self> {
        let context = EthSignatureBuilderContext::new(region_path, db)?;
        Ok(Self::new_inner(db, context))
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct EthSignatureBuilderContext {
    task_ty: Option<EthTerm>,
}

impl EthSignatureBuilderContext {
    fn new_generic() -> Self {
        Self { task_ty: None }
    }

    fn new(region_path: SynNodeRegionPath, db: &::salsa::Db) -> EthSignatureResult<Self> {
        let package_path = region_path.package_path(db);
        let task_ty = package_path.eth_signature(db)?.data(db).task_ty();
        Ok(Self { task_ty })
    }

    pub fn task_ty(&self) -> Option<EthTerm> {
        self.task_ty
    }
}

impl<'db> IsEthInstantiationContext<'db> for EthSignatureBuilderContext {
    fn reduce_ty_as_trai_item(
        &self,
        term: husky_eth_term::term::trai_for_ty_item::EthTypeAsTraitItem,
    ) -> EthTerm {
        todo!()
    }

    fn task_ty(&self) -> Option<EthTerm> {
        self.task_ty
    }
}
