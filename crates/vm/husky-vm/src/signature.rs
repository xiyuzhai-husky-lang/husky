use husky_word::{IdentMap, Identifier};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TySignature {
    Enum,
    Struct {
        fields: IdentMap<(Identifier, Binding)>,
    },
    Vec,
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct VMMembVarSignature {
//     pub ident: Identifier,
//     pub contract: MembVarContract,
// }
