mod fugitive_body;
mod major_items;
mod trai_for_ty_items;
mod trai_items;
mod ty_items;
mod ty_variants;

pub use self::fugitive_body::*;
pub use self::major_items::*;
pub use self::trai_for_ty_items::*;
pub use self::trai_items::*;
pub use self::ty_items::*;
pub use self::ty_variants::*;

use crate::*;
use husky_entity_kind::*;
use husky_entity_path::*;
use husky_token::*;
use parsec::*;

/// a possible type of ast children with specifications for parsing
pub(crate) trait IsAstChildren {
    /// specifies whether statements are allowed
    const ALLOW_STMT: AstResult<()>;

    /// specifies how to determine item kinds
    fn determine_item_kind(keyword_group: EntityKindKeywordGroup) -> AstResult<EntityKind>;

    // todo: add a checker for trivialness
    // for example, a function body might be filled with uses, which is not enough
}

/// ast children and entity path for a definition block
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum DefnBlock {
    Fugitive {
        path: FugitivePath,
        body: Option<FugitiveBody>,
    },
    Submodule {
        path: SubmoduleItemPath,
    },
    Type {
        path: TypePath,
        variants: Option<TypeVariants>,
    },
    Trait {
        path: TraitPath,
        items: Option<TraitItems>,
    },
    // doesn't have a path field because the impl block might be ill-formed
    AssocItem {
        body: Option<FugitiveBody>,
    },
}

impl DefnBlock {
    pub fn children(self) -> Option<AstIdxRange> {
        match self {
            DefnBlock::Fugitive { body, .. } => body.map(|v| v.ast_idx_range()),
            // in husky, there are no inline modules
            DefnBlock::Submodule { .. } => None,
            DefnBlock::Type { variants, .. } => variants.map(|v| v.ast_idx_range()),
            DefnBlock::Trait { items, .. } => items.map(|items| items.ast_idx_range()),
            DefnBlock::AssocItem { body } => body.map(|v| v.ast_idx_range()),
        }
    }

    /// only for non-associated entities
    #[inline(always)]
    pub fn item_path(self) -> Option<ItemPath> {
        match self {
            DefnBlock::Fugitive { path, .. } => Some(path.into()),
            DefnBlock::Submodule { path } => Some(path.into()),
            DefnBlock::Type { path, .. } => Some(path.into()),
            DefnBlock::Trait { path, .. } => Some(path.into()),
            DefnBlock::AssocItem { .. } => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum ImplBlockItems {
    Type(TypeItems),
    TraitForType(TraitForTypeItems),
}

impl ImplBlockItems {
    pub fn ast_idx_range(self) -> AstIdxRange {
        match self {
            ImplBlockItems::Type(items) => items.ast_idx_range(),
            ImplBlockItems::TraitForType(items) => items.ast_idx_range(),
        }
    }
}
