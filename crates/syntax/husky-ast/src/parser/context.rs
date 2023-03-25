use husky_entity_path::*;
use husky_entity_taxonomy::{
    AssociatedItemKind, EntityKind, ModuleItemKind, TraitItemKind, TypeItemKind, TypeKind,
};

use crate::{AstResult, OriginalAstError, INDENT_INCR};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AstContextKind {
    InsideTrait {
        module_item_path: ModuleItemPath,
    },
    InsideEnumLikeType {
        module_item_path: ModuleItemPath,
    },
    /// inside function, method or inline block or main or config
    InsideForm,
    InsideTypeImplBlock,
    InsideTypeAsTraitImplBlock,
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
                AssociatedItemKind::TypeItem(ty_item_kind) => match ty_item_kind {
                    TypeItemKind::MethodFn | TypeItemKind::AssociatedFn | TypeItemKind::Memo => {
                        AstContextKind::InsideForm
                    }
                },
                AssociatedItemKind::TraitItem(trai_item_kind) => match trai_item_kind {
                    TraitItemKind::MethodFn => {
                        // ad hoc
                        // todo: should check whether ends with ';' or ':'
                        AstContextKind::InsideForm
                    }
                    TraitItemKind::AssociatedType => AstContextKind::InsideNoChild,
                },
                AssociatedItemKind::TraitForTypeItem(trait_item_kind) => match trait_item_kind {
                    // ad hoc
                    // todo: should check whether ends with ';' or ':'
                    TraitItemKind::MethodFn => AstContextKind::InsideForm,
                    TraitItemKind::AssociatedType => todo!(),
                },
            },
            EntityKind::Variant => todo!(),
        }
    }

    pub(crate) fn allow_stmt(self) -> AstResult<()> {
        match self {
            AstContextKind::InsideTrait { module_item_path } => {
                Err(OriginalAstError::UnexpectedStmtInsideTrait)?
            }
            AstContextKind::InsideEnumLikeType { module_item_path } => todo!(),
            AstContextKind::InsideForm => Ok(()),
            AstContextKind::InsideTypeImplBlock | AstContextKind::InsideTypeAsTraitImplBlock => {
                Err(OriginalAstError::UnexpectedStmtInsideImplBlock)?
            }
            AstContextKind::InsideModule => Ok(()),
            AstContextKind::InsideMatchStmt => Ok(()),
            AstContextKind::InsideNoChild => Err(OriginalAstError::ExpectNothing)?,
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
        let indent = match parent {
            AstContextKind::InsideMatchStmt | AstContextKind::InsideEnumLikeType { .. } => {
                self.indent
            }
            AstContextKind::InsideTrait { .. }
            | AstContextKind::InsideForm
            | AstContextKind::InsideTypeImplBlock
            | AstContextKind::InsideTypeAsTraitImplBlock
            | AstContextKind::InsideModule
            | AstContextKind::InsideNoChild => self.indent + INDENT_INCR,
        };
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
