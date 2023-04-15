mod dyn_trai;
mod error;
mod trai_for_ty;
mod ty;

pub use self::error::*;

use super::*;
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyMethodDisambiguation {
    indirections: SmallVec<[FluffyMethodIndirection; 2]>,
    ty_path: TypePath,
    variant: FluffyMethodDisambiguationVariant,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyMethodIndirection {}

#[derive(Debug, PartialEq, Eq)]
pub enum FluffyMethodDisambiguationVariant {
    Type,
    Trait {
        trai_path: TraitPath,
        trai: FluffyTerm,
    },
}

impl FluffyTerm {
    pub fn method_ty(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyMethodTypeResult<(
        FluffyMethodDisambiguation,
        FluffyMethodTypeResult<FluffyTerm>,
    )> {
        if let Some((indirections, ty_path, ty_result)) = self.ty_method_ty(ident)? {
            return Ok((
                FluffyMethodDisambiguation {
                    indirections,
                    ty_path,
                    variant: FluffyMethodDisambiguationVariant::Type,
                },
                ty_result,
            ));
        }
        if let Some((indirections, ty_path, trai_path, trai, ty_result)) =
            self.trai_for_ty_method_ty(ident, available_traits)?
        {
            return Ok((
                FluffyMethodDisambiguation {
                    indirections,
                    ty_path,
                    variant: FluffyMethodDisambiguationVariant::Trait { trai_path, trai },
                },
                ty_result,
            ));
        }
        todo!()
    }
}
