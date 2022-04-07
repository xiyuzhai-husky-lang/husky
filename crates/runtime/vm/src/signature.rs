use vec_map::VecDict;
use word::CustomIdentifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TySignature {
    Enum,
    Struct {
        memb_vars: VecDict<CustomIdentifier, MembAccessContract>,
    },
    Vec,
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct VMMembVarSignature {
//     pub ident: CustomIdentifier,
//     pub contract: MembVarContract,
// }
