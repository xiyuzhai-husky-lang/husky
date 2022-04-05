mod opr;

use std::sync::Arc;

pub use opr::*;

use scope::{
    GenericPlaceholderKind, InputPlaceholder, InputSignature, RangedScope, ScopeKind, ScopePtr,
};
use vm::{InputContract, MembAccessContract};
use word::{CustomIdentifier, IdentMap};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MembAccessSignature {
    pub contract: MembAccessContract,
    pub ty: ScopePtr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembCallSignature {
    pub this_contract: InputContract,
    pub inputs: Vec<InputSignature>,
    pub output: ScopePtr,
    pub args: IdentMap<GenericPlaceholderKind>,
}
// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct MembFuncDecl {
//     this: EagerContract,
//     inputs: Vec<InputPlaceholder>,
//     output: RangedScope,
//     args: Vec<(CustomIdentifier, GenericPlaceholderKind)>,
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawEnumVariantKind {
    Constant,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RoutineKind {
    Test,
    Proc,
    Func,
    Def,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutineHead {
    pub routine_name: CustomIdentifier,
    pub generic_placeholders: IdentMap<GenericPlaceholderKind>,
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output: RangedScope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembRoutineHead {
    pub this_contract: InputContract,
    pub kind: RawMembRoutineKind,
    pub routine_name: CustomIdentifier,
    pub generics: IdentMap<GenericPlaceholderKind>,
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output: RangedScope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawMembRoutineKind {
    Proc,
    Func,
}

impl Into<MembCallSignature> for &MembRoutineHead {
    fn into(self) -> MembCallSignature {
        MembCallSignature {
            this_contract: self.this_contract,
            inputs: self
                .input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.into())
                .collect(),
            output: self.output.scope,
            args: self.generics.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MembType {
    pub contract: MembAccessContract,
    pub ty: ScopePtr,
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub struct InputType {
//     pub contract: Contract,
//     pub ty: RangedScope,
// }

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BuiltinScopeData {
    scope_kind: ScopeKind,
    subscopes: [(String, &'static BuiltinScopeData)],
}
