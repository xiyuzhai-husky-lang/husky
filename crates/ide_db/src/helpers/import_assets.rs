//! Look up accessible paths for items.
use hir::{
    AsAssocItem, AssocItem, AssocItemContainer, Crate, ItemInNs, MacroDef, ModPath, Module,
    ModuleDef, PathResolution, PrefixKind, ScopeDef, Semantics, Type,
};
use itertools::Itertools;
use rustc_hash::FxHashSet;
use syntax::{ast, SyntaxNode};

use crate::{
    helpers::get_path_in_derive_attr,
    items_locator::{self, AssocItemSearch, DEFAULT_QUERY_SEARCH_LIMIT},
    RootDatabase,
};

use super::item_name;

/// A candidate for import, derived during various IDE activities:
/// * completion with imports on the fly proposals
/// * completion edit resolve requests
/// * assists
/// * etc.
#[derive(Debug)]
pub enum ImportCandidate {
    /// A path, qualified (`std::collections::HashMap`) or not (`HashMap`).
    Path(PathImportCandidate),
    /// A trait associated function (with no self parameter) or an associated constant.
    /// For 'test_mod::TestEnum::test_function', `ty` is the `test_mod::TestEnum` expression type
    /// and `name` is the `test_function`
    TraitAssocItem(TraitImportCandidate),
    /// A trait method with self parameter.
    /// For 'test_enum.test_method()', `ty` is the `test_enum` expression type
    /// and `name` is the `test_method`
    TraitMethod(TraitImportCandidate),
}

/// A trait import needed for a given associated item access.
/// For `some::path::SomeStruct::ASSOC_`, contains the
/// type of `some::path::SomeStruct` and `ASSOC_` as the item name.
#[derive(Debug)]
pub struct TraitImportCandidate {
    /// A type of the item that has the associated item accessed at.
    pub receiver_ty: Type,
    /// The associated item name that the trait to import should contain.
    pub assoc_item_name: NameToImport,
}

/// Path import for a given name, qualified or not.
#[derive(Debug)]
pub struct PathImportCandidate {
    /// Optional qualifier before name.
    pub qualifier: Option<FirstSegmentUnresolved>,
    /// The name the item (struct, trait, enum, etc.) should have.
    pub name: NameToImport,
}

/// A qualifier that has a first segment and it's unresolved.
#[derive(Debug)]
pub struct FirstSegmentUnresolved {
    fist_segment: ast::NameRef,
    full_qualifier: ast::Path,
}

/// A name that will be used during item lookups.
#[derive(Debug, Clone)]
pub enum NameToImport {
    /// Requires items with names that exactly match the given string, case-sensitive.
    Exact(String),
    /// Requires items with names that case-insensitively contain all letters from the string,
    /// in the same order, but not necessary adjacent.
    Fuzzy(String),
}

impl NameToImport {
    pub fn text(&self) -> &str {
        match self {
            NameToImport::Exact(text) => text.as_str(),
            NameToImport::Fuzzy(text) => text.as_str(),
        }
    }
}

/// A struct to find imports in the project, given a certain name (or its part) and the context.
#[derive(Debug)]
pub struct ImportAssets {
    import_candidate: ImportCandidate,
    candidate_node: SyntaxNode,
    module_with_candidate: Module,
}

impl ImportAssets {
    pub fn for_method_call(
        method_call: &ast::MethodCallExpr,
        sema: &Semantics<RootDatabase>,
    ) -> Option<Self> {
        todo!()
    }

    pub fn for_exact_path(
        fully_qualified_path: &ast::Path,
        sema: &Semantics<RootDatabase>,
    ) -> Option<Self> {
        todo!()
    }

    pub fn for_ident_pat(sema: &Semantics<RootDatabase>, pat: &ast::IdentPat) -> Option<Self> {
        todo!()
    }

    pub fn for_derive_ident(sema: &Semantics<RootDatabase>, ident: &ast::Ident) -> Option<Self> {
        todo!()
    }

    pub fn for_fuzzy_path(
        module_with_candidate: Module,
        qualifier: Option<ast::Path>,
        fuzzy_name: String,
        sema: &Semantics<RootDatabase>,
        candidate_node: SyntaxNode,
    ) -> Option<Self> {
        Some(Self {
            import_candidate: ImportCandidate::for_fuzzy_path(qualifier, fuzzy_name, sema)?,
            module_with_candidate,
            candidate_node,
        })
    }

    pub fn for_fuzzy_method_call(
        module_with_method_call: Module,
        receiver_ty: Type,
        fuzzy_method_name: String,
        candidate_node: SyntaxNode,
    ) -> Option<Self> {
        Some(Self {
            import_candidate: ImportCandidate::TraitMethod(TraitImportCandidate {
                receiver_ty,
                assoc_item_name: NameToImport::Fuzzy(fuzzy_method_name),
            }),
            module_with_candidate: module_with_method_call,
            candidate_node,
        })
    }
}

/// An import (not necessary the only one) that corresponds a certain given [`PathImportCandidate`].
/// (the structure is not entirely correct, since there can be situations requiring two imports, see FIXME below for the details)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocatedImport {
    /// The path to use in the `use` statement for a given candidate to be imported.
    pub import_path: ModPath,
    /// An item that will be imported with the import path given.
    pub item_to_import: ItemInNs,
    /// The path import candidate, resolved.
    ///
    /// Not necessary matches the import:
    /// For any associated constant from the trait, we try to access as `some::path::SomeStruct::ASSOC_`
    /// the original item is the associated constant, but the import has to be a trait that
    /// defines this constant.
    pub original_item: ItemInNs,
    /// A path of the original item.
    pub original_path: Option<ModPath>,
}

impl LocatedImport {
    pub fn new(
        import_path: ModPath,
        item_to_import: ItemInNs,
        original_item: ItemInNs,
        original_path: Option<ModPath>,
    ) -> Self {
        Self {
            import_path,
            item_to_import,
            original_item,
            original_path,
        }
    }
}

impl ImportAssets {
    pub fn import_candidate(&self) -> &ImportCandidate {
        &self.import_candidate
    }

    pub fn search_for_imports(
        &self,
        sema: &Semantics<RootDatabase>,
        prefix_kind: PrefixKind,
    ) -> Vec<LocatedImport> {
        let _p = profile::span("import_assets::search_for_imports");
        self.search_for(sema, Some(prefix_kind))
    }

    /// This may return non-absolute paths if a part of the returned path is already imported into scope.
    pub fn search_for_relative_paths(&self, sema: &Semantics<RootDatabase>) -> Vec<LocatedImport> {
        let _p = profile::span("import_assets::search_for_relative_paths");
        self.search_for(sema, None)
    }

    fn search_for(
        &self,
        sema: &Semantics<RootDatabase>,
        prefixed: Option<PrefixKind>,
    ) -> Vec<LocatedImport> {
        todo!()
    }

    fn scope_definitions(&self, sema: &Semantics<RootDatabase>) -> FxHashSet<ScopeDef> {
        todo!()
    }
}

fn path_applicable_imports(
    sema: &Semantics<RootDatabase>,
    current_crate: Crate,
    path_candidate: &PathImportCandidate,
    mod_path: impl Fn(ItemInNs) -> Option<ModPath> + Copy,
) -> FxHashSet<LocatedImport> {
    todo!()
}

fn import_for_item(
    db: &RootDatabase,
    mod_path: impl Fn(ItemInNs) -> Option<ModPath>,
    unresolved_first_segment: &str,
    unresolved_qualifier: &str,
    original_item: ItemInNs,
) -> Option<LocatedImport> {
    todo!()
}

pub fn item_for_path_search(db: &RootDatabase, item: ItemInNs) -> Option<ItemInNs> {
    todo!()
}

fn find_import_for_segment(
    db: &RootDatabase,
    original_item: ItemInNs,
    unresolved_first_segment: &str,
) -> Option<ItemInNs> {
    todo!()
}

fn module_with_segment_name(
    db: &RootDatabase,
    segment_name: &str,
    candidate: ItemInNs,
) -> Option<Module> {
    todo!()
}

fn trait_applicable_items(
    sema: &Semantics<RootDatabase>,
    current_crate: Crate,
    trait_candidate: &TraitImportCandidate,
    trait_assoc_item: bool,
    mod_path: impl Fn(ItemInNs) -> Option<ModPath>,
) -> FxHashSet<LocatedImport> {
    todo!()
}

fn assoc_to_item(assoc: AssocItem) -> ItemInNs {
    todo!()
}

fn get_mod_path(
    db: &RootDatabase,
    item_to_search: ItemInNs,
    module_with_candidate: &Module,
    prefixed: Option<PrefixKind>,
) -> Option<ModPath> {
    todo!()
}

impl ImportCandidate {
    fn for_method_call(
        sema: &Semantics<RootDatabase>,
        method_call: &ast::MethodCallExpr,
    ) -> Option<Self> {
        todo!()
    }

    fn for_regular_path(sema: &Semantics<RootDatabase>, path: &ast::Path) -> Option<Self> {
        todo!()
    }

    fn for_name(sema: &Semantics<RootDatabase>, name: &ast::Name) -> Option<Self> {
        todo!()
    }

    fn for_fuzzy_path(
        qualifier: Option<ast::Path>,
        fuzzy_name: String,
        sema: &Semantics<RootDatabase>,
    ) -> Option<Self> {
        path_import_candidate(sema, qualifier, NameToImport::Fuzzy(fuzzy_name))
    }
}

fn path_import_candidate(
    sema: &Semantics<RootDatabase>,
    qualifier: Option<ast::Path>,
    name: NameToImport,
) -> Option<ImportCandidate> {
    todo!()
}

fn item_as_assoc(db: &RootDatabase, item: ItemInNs) -> Option<AssocItem> {
    todo!()
}
