mod dyn_trai;
mod error;
mod trai_for_ty;
mod ty;

pub use self::error::*;

use super::*;
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyMethodType {
    indirections: SmallVec<[FluffyMethodIndirection; 2]>,
    disambiguation: FluffyMethodDisambiguation,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyMethodIndirection {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyMethodDisambiguation {
    Type,
    Trait { trai: FluffyTerm },
}

impl FluffyTerm {
    pub fn method_ty(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyMethodTypeResult<FluffyMethodType> {
        if let Some(ty_method_ty) = self.ty_method_ty(ident)? {
            return Ok(ty_method_ty);
        }
        if let Some(trai_for_ty_method_ty) = self.trai_for_ty_method_ty(ident, available_traits)? {
            return Ok(trai_for_ty_method_ty);
        }
        todo!()
    }
}
