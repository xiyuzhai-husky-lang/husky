mod opr;

use std::sync::Arc;

pub use opr::*;

use entity_route::{EntityRouteKind, EntityRoutePtr, GenericPlaceholder, RangedEntityRoute};
use vm::{InputContract, MembAccessContract};
use word::{CustomIdentifier, IdentMap};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnumVariantKind {
    Constant,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RoutineKind {
    Test,
    Proc,
    Func,
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct RoutineHead {
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct MembRoutineHead {
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawMembRoutineKind {
    Proc,
    Func,
}

// impl Into<MembCallDecl> for &MembRoutineHead {
//     fn into(self) -> MembCallDecl {
//         MembCallDecl {
//             this_contract: self.this_contract,
//             inputs: self
//                 .input_placeholders
//                 .iter()
//                 .map(|input_placeholder| input_placeholder.into())
//                 .collect(),
//             output: self.output.route,
//             args: self.generics.clone(),
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MembType {
    pub contract: MembAccessContract,
    pub ty: EntityRoutePtr,
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub struct InputType {
//     pub contract: Contract,
//     pub ty: RangedScope,
// }

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BuiltinScopeData {
    scope_kind: EntityRouteKind,
    subscopes: [(String, &'static BuiltinScopeData)],
}
