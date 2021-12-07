//! See [`NavigationTarget`].

use std::fmt;

use common::*;

use either::Either;
use hir::{AssocItem, Documentation, FieldSource, HirDisplay, InFile, ModuleSource, Semantics};
use ide_db::{defs::Definition, IdeDatabase};
use ide_db::{
    file_db::{FileID, FileRange},
    symbol_index::FileSymbolKind,
    SymbolKind,
};
use syntax::{ast, SmolStr};

use crate::FileSymbol;

/// `NavigationTarget` represents an element in the editor's UI which you can
/// click on to navigate to a particular piece of code.
///
/// Typically, a `NavigationTarget` corresponds to some element in the source
/// code, like a function or a struct, but this is not strictly required.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NavigationTarget {
    pub file_id: FileID,
    /// Range which encompasses the whole element.
    ///
    /// Should include body, doc comments, attributes, etc.
    ///
    /// Clients should use this range to answer "is the cursor inside the
    /// element?" question.
    pub full_range: TextRange,
    /// A "most interesting" range within the `full_range`.
    ///
    /// Typically, `full_range` is the whole syntax node, including doc
    /// comments, and `focus_range` is the range of the identifier.
    ///
    /// Clients should place the cursor on this range when navigating to this target.
    pub focus_range: Option<TextRange>,
    pub name: SmolStr,
    pub kind: Option<SymbolKind>,
    pub container_name: Option<SmolStr>,
    pub description: Option<String>,
    pub docs: Option<Documentation>,
}

impl fmt::Debug for NavigationTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub(crate) trait ToNav {
    fn to_nav(&self, db: &IdeDatabase) -> NavigationTarget;
}

pub(crate) trait TryToNav {
    fn try_to_nav(&self, db: &IdeDatabase) -> Option<NavigationTarget>;
}

impl<T: TryToNav, U: TryToNav> TryToNav for Either<T, U> {
    fn try_to_nav(&self, db: &IdeDatabase) -> Option<NavigationTarget> {
        match self {
            Either::Left(it) => it.try_to_nav(db),
            Either::Right(it) => it.try_to_nav(db),
        }
    }
}

impl NavigationTarget {
    pub fn focus_or_full_range(&self) -> TextRange {
        self.focus_range.unwrap_or(self.full_range)
    }

    pub(crate) fn from_module_to_decl(db: &IdeDatabase, module: hir::Module) -> NavigationTarget {
        todo!()
    }

    fn from_syntax(
        file_id: FileID,
        name: SmolStr,
        focus_range: Option<TextRange>,
        full_range: TextRange,
        kind: SymbolKind,
    ) -> NavigationTarget {
        NavigationTarget {
            file_id,
            name,
            kind: Some(kind),
            full_range,
            focus_range,
            container_name: None,
            description: None,
            docs: None,
        }
    }
}

impl ToNav for FileSymbol {
    fn to_nav(&self, db: &IdeDatabase) -> NavigationTarget {
        NavigationTarget {
            file_id: self.file_id,
            name: self.name.clone(),
            kind: Some(match self.kind {
                FileSymbolKind::Function => SymbolKind::Function,
                FileSymbolKind::Struct => SymbolKind::Struct,
                FileSymbolKind::Enum => SymbolKind::Enum,
                FileSymbolKind::Trait => SymbolKind::Trait,
                FileSymbolKind::Module => SymbolKind::Module,
                FileSymbolKind::TypeAlias => SymbolKind::TypeAlias,
                FileSymbolKind::Const => SymbolKind::Const,
                FileSymbolKind::Static => SymbolKind::Static,
                FileSymbolKind::Macro => SymbolKind::Macro,
                FileSymbolKind::Union => SymbolKind::Union,
            }),
            full_range: self.range,
            focus_range: self.name_range,
            container_name: self.container_name.clone(),
            description: description_from_symbol(db, self),
            docs: None,
        }
    }
}

impl TryToNav for Definition {
    fn try_to_nav(&self, db: &IdeDatabase) -> Option<NavigationTarget> {
        todo!()
    }
}

impl TryToNav for hir::ModuleDef {
    fn try_to_nav(&self, db: &IdeDatabase) -> Option<NavigationTarget> {
        todo!()
    }
}

pub(crate) trait ToNavFromAst {
    const KIND: SymbolKind;
}
impl ToNavFromAst for hir::Function {
    const KIND: SymbolKind = SymbolKind::Function;
}
impl ToNavFromAst for hir::Const {
    const KIND: SymbolKind = SymbolKind::Const;
}
impl ToNavFromAst for hir::Static {
    const KIND: SymbolKind = SymbolKind::Static;
}
impl ToNavFromAst for hir::Struct {
    const KIND: SymbolKind = SymbolKind::Struct;
}
impl ToNavFromAst for hir::Enum {
    const KIND: SymbolKind = SymbolKind::Enum;
}
impl ToNavFromAst for hir::Variant {
    const KIND: SymbolKind = SymbolKind::Variant;
}
impl ToNavFromAst for hir::Union {
    const KIND: SymbolKind = SymbolKind::Union;
}
impl ToNavFromAst for hir::TypeAlias {
    const KIND: SymbolKind = SymbolKind::TypeAlias;
}
impl ToNavFromAst for hir::Trait {
    const KIND: SymbolKind = SymbolKind::Trait;
}

impl ToNav for hir::Module {
    fn to_nav(&self, db: &IdeDatabase) -> NavigationTarget {
        todo!()
    }
}

impl TryToNav for hir::Impl {
    fn try_to_nav(&self, db: &IdeDatabase) -> Option<NavigationTarget> {
        todo!()
    }
}

impl TryToNav for hir::Field {
    fn try_to_nav(&self, db: &IdeDatabase) -> Option<NavigationTarget> {
        todo!()
    }
}

impl TryToNav for hir::MacroDef {
    fn try_to_nav(&self, db: &IdeDatabase) -> Option<NavigationTarget> {
        todo!()
    }
}

impl TryToNav for hir::Adt {
    fn try_to_nav(&self, db: &IdeDatabase) -> Option<NavigationTarget> {
        todo!()
    }
}

impl TryToNav for hir::AssocItem {
    fn try_to_nav(&self, db: &IdeDatabase) -> Option<NavigationTarget> {
        todo!()
    }
}

impl TryToNav for hir::GenericParam {
    fn try_to_nav(&self, db: &IdeDatabase) -> Option<NavigationTarget> {
        todo!()
    }
}

impl ToNav for hir::Local {
    fn to_nav(&self, db: &IdeDatabase) -> NavigationTarget {
        todo!()
    }
}

impl ToNav for hir::Label {
    fn to_nav(&self, db: &IdeDatabase) -> NavigationTarget {
        todo!()
    }
}

impl TryToNav for hir::TypeParam {
    fn try_to_nav(&self, db: &IdeDatabase) -> Option<NavigationTarget> {
        todo!()
    }
}

impl TryToNav for hir::ConstParam {
    fn try_to_nav(&self, db: &IdeDatabase) -> Option<NavigationTarget> {
        todo!()
    }
}

/// Get a description of a symbol.
///
/// e.g. `struct Name`, `enum Name`, `fn Name`
pub(crate) fn description_from_symbol(db: &IdeDatabase, symbol: &FileSymbol) -> Option<String> {
    todo!()
}
