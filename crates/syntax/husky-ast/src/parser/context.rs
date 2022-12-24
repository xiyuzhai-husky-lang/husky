use husky_entity_path::*;
use husky_text::TextRange;

use crate::INDENT_INCR;

#[derive(Debug, Clone, Copy)]
pub(super) enum AstParent {
    /// inside function, method or inline block
    Form,
    EnumLike,
    TraitOrNonEnumLikeType {
        module_item_path: ModuleItemPath,
    },
    Impl,
    /// module level
    Module,
    /// ```python
    /// match token with
    /// | 0 => ...
    /// | 1 => ...
    /// ```
    MatchStmt,
    NoChild,
}

impl AstParent {
    pub(super) fn module_item_path(self) -> Option<ModuleItemPath> {
        match self {
            AstParent::Form => todo!(),
            AstParent::EnumLike => todo!(),
            AstParent::TraitOrNonEnumLikeType { module_item_path } => Some(module_item_path),
            AstParent::Impl => None,
            AstParent::Module => todo!(),
            AstParent::MatchStmt => todo!(),
            AstParent::NoChild => todo!(),
        }
    }
}

pub(super) struct Context {
    parent: AstParent,
    indent: u32,
}

impl Context {
    pub(crate) fn new_module() -> Self {
        Self {
            parent: AstParent::Module,
            indent: 0,
        }
    }

    pub(crate) fn indent(&self) -> u32 {
        self.indent
    }

    #[inline(always)]
    pub(super) fn subcontext(&self, parent: AstParent) -> Self {
        let indent = self.indent + INDENT_INCR;
        Self { indent, parent }
    }

    pub(super) fn parent(&self) -> AstParent {
        self.parent
    }
}

pub(super) enum SubcontextKind {
    FormBlock,
    AssociatedBody,
    Nothing,
    MatchCase,
    ModuleItemVariant,
}
