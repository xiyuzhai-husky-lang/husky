use super::*;
use crate::{rank::Rank, symbol::Symbol, token::ident::Ident};
use husky_cybertron::seq::Seq;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeSignature {
    key: TypeSignatureKey,
    ty: Type,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeSignatureKey {
    CallArgument { function: Symbol, rank: Rank },
    Field { rank: Rank },
}

fn ty_signatures() -> Seq<Option<TypeSignature>> {
    todo!()
}
