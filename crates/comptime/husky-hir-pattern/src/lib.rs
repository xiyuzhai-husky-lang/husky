mod build;
pub mod db;

use crate::db::*;
use husky_coword::*;
use husky_entity_path::*;
use husky_hir_ty::*;
use husky_term_prelude::*;
use smallvec::SmallVec;

#[salsa::interned(db = HirPatternDb, jar = HirPatternJar)]
pub struct HirPattern {
    #[return_ref]
    data: HirPatternData,
    ty: HirType,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirPatternData {
    /// example: `1`
    Literal(/* todo */),
    /// example: `a`
    Ident {
        symbol_modifier: Option<SymbolModifier>,
        ident: Ident,
    },
    /// example: `A::B`
    Entity(ItemPath),
    /// example: `(a, b)`
    Tuple {
        name: Option<ItemPath>,
        fields: SmallVec<[HirPattern; 2]>,
    },
    /// example: `C { .. }`
    Props {
        name: Option<ItemPath>,
        fields: SmallVec<[HirPattern; 4]>,
    },
    /// example: `A | B | C { .. }`
    OneOf { options: SmallVec<[HirPattern; 4]> },
    /// example: `x @ 1..9`
    Binding {
        ident: Ident,
        /// example: `1..9`
        src: HirPattern,
    },
    /// example: `1..9`
    Range { start: HirPattern, end: HirPattern },
}
