use vec_map::VecMap;
use word::CustomIdentifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TySignature {
    Enum,
    Struct {
        memb_vars: VecMap<CustomIdentifier, MembAccessContract>,
    },
    Vec,
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct VMMembVarSignature {
//     pub ident: CustomIdentifier,
//     pub contract: MembVarContract,
// }
