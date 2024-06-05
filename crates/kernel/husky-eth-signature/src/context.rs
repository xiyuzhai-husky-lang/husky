use husky_entity_path::path::ItemPath;
use husky_entity_tree::region_path::SynNodeRegionPath;
use husky_eth_term::{instantiation::IsEthInstantiationContext, term::EthTerm, EthTermResult};

use crate::signature::HasEthSignature;

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

    pub fn new(region_path: SynNodeRegionPath, db: &::salsa::Db) -> Self {
        let context = EthSignatureBuilderContext::new(region_path, db);
        Self::new_inner(db, context)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct EthSignatureBuilderContext {
    task_ty: Result<Option<EthTerm>, ()>,
}

impl EthSignatureBuilderContext {
    fn new_generic() -> Self {
        Self { task_ty: Ok(None) }
    }

    fn new(region_path: SynNodeRegionPath, db: &::salsa::Db) -> Self {
        let package_path = region_path.package_path(db);
        let task_ty = match package_path.eth_signature(db) {
            Ok(signature) => Ok(signature.data(db).task_ty()),
            Err(_) => Err(()),
        };
        Self { task_ty }
    }
}

impl<'db> IsEthInstantiationContext<'db> for EthSignatureBuilderContext {
    fn reduce_ty_as_trai_item(
        &self,
        term: husky_eth_term::term::trai_for_ty_item::EthTypeAsTraitItem,
    ) -> EthTerm {
        todo!()
    }

    /// returns Ok(None) if there is no dependency on task type
    fn task_ty(
        &self,
        item_path: husky_entity_path::path::ItemPath,
    ) -> EthTermResult<Option<EthTerm>> {
        todo!("check item_path dependency")
    }
}
