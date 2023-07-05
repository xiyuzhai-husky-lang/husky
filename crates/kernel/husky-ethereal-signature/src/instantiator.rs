use crate::*;
use vec_like::VecPairMap;

pub(crate) struct Instantiator {
    symbol_map: VecPairMap<EtherealTermSymbol, EtherealTermSymbolInstantiationProgress>,
}

impl Instantiator {
    pub(crate) fn new() -> Self {
        Self {
            symbol_map: todo!(),
        }
    }
}

pub(crate) enum EtherealTermSymbolInstantiationProgress {
    Unresolved,
}

impl Default for EtherealTermSymbolInstantiationProgress {
    fn default() -> Self {
        EtherealTermSymbolInstantiationProgress::Unresolved
    }
}

pub(crate) trait Instantiate {
    type Target;

    fn instantiate(&self, instantiator: Instantiator) -> Self::Target;
}
