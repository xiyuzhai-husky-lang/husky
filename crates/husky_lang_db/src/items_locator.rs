//! This module has the functionality to search the project and its dependencies for a certain item,
//! by its name and a few criteria.
//! The main reason for this module to exist is the fact that project's items and dependencies' items
//! are located in different caches, with different APIs.
use either::Either;
use hir::{AsAssocItem, Crate, ItemInNs, Semantics};
use limit::Limit;
use syntax::ast;

use crate::{
    defs::{Definition, NameClass},
    helpers::import_assets::NameToImport,
    symbol_index::{self, FileSymbol},
    HuskyLangDatabase,
};

/// A value to use, when uncertain which limit to pick.
pub static DEFAULT_QUERY_SEARCH_LIMIT: Limit = Limit::new(40);

/// Three possible ways to search for the name in associated and/or other items.
#[derive(Debug, Clone, Copy)]
pub enum AssocItemSearch {
    /// Search for the name in both associated and other items.
    Include,
    /// Search for the name in other items only.
    Exclude,
    /// Search for the name in the associated items only.
    AssocItemsOnly,
}

fn get_name_definition(
    sema: &Semantics<'_, HuskyLangDatabase>,
    import_candidate: &FileSymbol,
) -> Option<Definition> {
    todo!()
}

fn is_assoc_item(item: ItemInNs, db: &HuskyLangDatabase) -> bool {
    todo!()
}
