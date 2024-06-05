use husky_eth_term::instantiation::IsEthInstantiationContext;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct EthSignatureBuilderContext {}

impl<'db> IsEthInstantiationContext<'db> for EthSignatureBuilderContext {}
