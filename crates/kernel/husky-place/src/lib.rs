pub mod jar;
pub mod place;

use husky_coword::Ident;
use husky_syn_expr::CurrentSynSymbolIdx;
use shifted_unsigned_int::ShiftedU32;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct PlaceRegistry {
    infos: Vec<PlaceInfo>,
    next: ShiftedU32,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum PlaceInfo {
    SelfValue,
    Parameter {
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        ident: Ident,
    },
    Variable {
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        ident: Ident,
    },
}
