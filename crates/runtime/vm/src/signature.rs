use vec_dict::VecDict;
use word::{CustomIdentifier, IdentDict};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TySignature {
    Enum,
    Struct {
        field_vars: IdentDict<(CustomIdentifier, FieldContract)>,
    },
    Vec,
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct VMMembVarSignature {
//     pub ident: CustomIdentifier,
//     pub contract: MembVarContract,
// }
