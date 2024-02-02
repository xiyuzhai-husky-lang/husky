use crate::SymbolDecTerm;
use husky_coword::Ident;
use vec_like::VecPairMap;

pub(crate) struct DecTermShowContext {
    entries: VecPairMap<SymbolDecTerm, Ident>,
}
