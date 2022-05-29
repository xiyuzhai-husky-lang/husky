mod opr;

use std::sync::Arc;

pub use opr::*;

use entity_route::{EntityRouteKind, EntityRoutePtr, RangedEntityRoute};
use vm::{InputLiason, MemberLiason};
use word::{CustomIdentifier, IdentDict};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MembType {
    pub contract: MemberLiason,
    pub ty: EntityRoutePtr,
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub struct InputType {
//     pub contract: Contract,
//     pub ty: RangedScope,
// }
