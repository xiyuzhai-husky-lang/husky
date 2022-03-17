use word::CustomIdentifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VMTySignature {
    Enum,
    Struct { memb_vars: Vec<VMMembVarSignature> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VMMembVarSignature {
    pub ident: CustomIdentifier,
    pub contract: MembVarContract,
}
