use husky_entity_path::*;
use husky_entity_taxonomy::{EntityKind, ModuleItemKind, TypeKind};
use husky_text::TextRange;

use crate::INDENT_INCR;

#[derive(Debug, Clone, Copy)]
pub(super) enum AstContextKind {
    InsideTrait {
        module_item_path: ModuleItemPath,
    },
    InsideEnumLikeType {
        module_item_path: ModuleItemPath,
    },
    /// inside function, method or inline block or main or config
    InsideForm,
    InsideImpl,
    /// module level
    InsideModule,
    /// ```python
    /// match token with
    /// | 0 => ...
    /// | 1 => ...
    /// ```
    InsideMatchStmt,
    InsideNoChild,
}

impl AstContextKind {
    pub(super) fn module_item_path(self) -> Option<ModuleItemPath> {
        match self {
            AstContextKind::InsideTrait { module_item_path }
            | AstContextKind::InsideEnumLikeType { module_item_path } => Some(module_item_path),
            _ => None,
        }
    }

    pub(super) fn inside_defn(entity_kind: EntityKind, entity_path: Option<EntityPath>) -> Self {
        match entity_kind {
            EntityKind::Module => AstContextKind::InsideNoChild,
            EntityKind::ModuleItem {
                module_item_kind, ..
            } => match module_item_kind {
                ModuleItemKind::Type(_) => AstContextKind::InsideNoChild,
                ModuleItemKind::Trait => AstContextKind::InsideTrait {
                    module_item_path: match entity_path {
                        Some(EntityPath::ModuleItem(module_item_path)) => module_item_path,
                        _ => unreachable!(),
                    },
                },
                ModuleItemKind::Form(_) => AstContextKind::InsideForm,
            },
            EntityKind::AssociatedItem {
                associated_item_kind: item_kind,
            } => match item_kind {
                ModuleItemKind::Type(type_kind) => match type_kind {
                    TypeKind::Enum | TypeKind::Inductive => todo!(),
                    TypeKind::Record
                    | TypeKind::Struct
                    | TypeKind::Structure
                    | TypeKind::Foreign => AstContextKind::InsideNoChild,
                },
                ModuleItemKind::Trait => todo!(),
                ModuleItemKind::Form(_) => AstContextKind::InsideForm,
            },
            EntityKind::Variant => todo!(),
        }
    }
}

pub(super) struct Context {
    inside: AstContextKind,
    indent: u32,
}

impl Context {
    pub(crate) fn new_module() -> Self {
        Self {
            inside: AstContextKind::InsideModule,
            indent: 0,
        }
    }

    pub(crate) fn indent(&self) -> u32 {
        self.indent
    }

    #[inline(always)]
    pub(super) fn subcontext(&self, parent: AstContextKind) -> Self {
        let indent = self.indent + INDENT_INCR;
        Self {
            indent,
            inside: parent,
        }
    }

    pub(super) fn kind(&self) -> AstContextKind {
        self.inside
    }
}

pub(super) enum SubcontextKind {
    FormBlock,
    AssociatedBody,
    Nothing,
    MatchCase,
    ModuleItemVariant,
}
