use husky_word::{CustomIdentifier, IdentDict};
use vec_like::VecMap;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TySignature {
    Enum,
    Struct {
        fields: IdentDict<(CustomIdentifier, Binding)>,
    },
    Vec,
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct VMMembVarSignature {
//     pub ident: CustomIdentifier,
//     pub contract: MembVarContract,
// }
