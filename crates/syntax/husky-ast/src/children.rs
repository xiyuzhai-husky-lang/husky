mod form_body;
mod module_items;
mod trai_for_ty_items;
mod trai_items;
mod ty_items;
mod variants;

use husky_entity_path::{FormPath, TraitPath, TypePath};

pub use self::form_body::*;
pub use self::module_items::*;
pub use self::trai_for_ty_items::*;
pub use self::trai_items::*;
pub use self::ty_items::*;
pub use self::variants::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum DefnBlock {
    Form {
        path: FormPath,
        body: FormBody,
    },
    Submodule {
        path: ModulePath,
    },
    Type {
        path: TypePath,
        variants: Option<TypeVariants>,
    },
    Trait {
        path: TraitPath,
        items: TraitItems,
    },
    // doesn't have a path field because the impl block might be ill-formed
    AssociatedItem {
        body: FormBody,
    },
}

impl DefnBlock {
    pub fn ast_idx_range(self) -> AstIdxRange {
        todo!()
    }

    pub fn form_body(self) -> Option<FormBody> {
        todo!()
    }

    /// only for non-associated entities
    pub fn entity_path(self) -> Option<EntityPath> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplBlockItems {}

impl ImplBlockItems {
    pub fn ast_idx_range(self) -> AstIdxRange {
        todo!()
    }
}
