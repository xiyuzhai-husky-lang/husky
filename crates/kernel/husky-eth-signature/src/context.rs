use husky_eth_term::instantiation::IsEthInstantiationContext;

#[salsa::interned(constructor = new)]
pub struct EthSignatureBuilderContextItd {
    #[return_ref]
    pub context: EthSignatureBuilderContext,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct EthSignatureBuilderContext {}

impl<'db> IsEthInstantiationContext<'db> for EthSignatureBuilderContext {
    fn reduce_ty_as_trai_item(
        &self,
        term: husky_eth_term::term::trai_for_ty_item::EthTypeAsTraitItem,
    ) -> husky_eth_term::term::EthTerm {
        todo!()
    }

    /// returns Ok(None) if there is no dependency on task type
    fn task_ty(
        &self,
        item_path: husky_entity_path::path::ItemPath,
    ) -> husky_eth_term::EthTermResult<Option<husky_eth_term::term::EthTerm>> {
        todo!()
    }
}
