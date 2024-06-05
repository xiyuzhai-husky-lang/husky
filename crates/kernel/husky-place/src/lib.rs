pub mod jar;
pub mod place;

use husky_coword::Ident;
use husky_syn_expr::variable::CurrentVariableIdx;
use shifted_unsigned_int::ShiftedU32;

#[salsa::derive_debug_with_db]
#[derive(Default, Debug, PartialEq, Eq)]
pub struct PlaceRegistry {
    infos: Vec<PlaceInfo>,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum PlaceInfo {
    SelfValue,
    Parameter {
        current_variable_idx: CurrentVariableIdx,
        ident: Ident,
    },
    Variable {
        current_variable_idx: CurrentVariableIdx,
        ident: Ident,
    },
}
