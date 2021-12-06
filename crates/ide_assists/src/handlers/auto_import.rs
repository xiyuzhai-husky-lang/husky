use common::*;

use ide_db::helpers::{
    import_assets::{ImportAssets, ImportCandidate},
    insert_use::{insert_use, ImportScope},
    mod_path_to_ast,
};
use syntax::{ast, NodeOrToken, SyntaxElement};

use crate::{AssistContext, AssistId, AssistKind, Assists, GroupLabel};

// Feature: Auto Import
//
// Using the `auto-import` assist it is possible to insert missing imports for unresolved items.
// When inserting an import it will do so in a structured manner by keeping imports grouped,
// separated by a newline in the following order:
//
// - `std` and `core`
// - External Crates
// - Current Crate, paths prefixed by `crate`
// - Current Module, paths prefixed by `self`
// - Super Module, paths prefixed by `super`
//
// Example:
// ```rust
// use std::fs::File;
//
// use itertools::Itertools;
// use syntax::ast;
//
// use crate::utils::insert_use;
//
// use self::auto_import;
//
// use super::AssistContext;
// ```
//
// .Import Granularity
//
// It is possible to configure how use-trees are merged with the `importGranularity` setting.
// It has the following configurations:
//
// - `crate`: Merge imports from the same crate into a single use statement. This kind of
//  nesting is only supported in Rust versions later than 1.24.
// - `module`: Merge imports from the same module into a single use statement.
// - `item`: Don't merge imports at all, creating one import per item.
// - `preserve`: Do not change the granularity of any imports. For auto-import this has the same
//  effect as `item`.
//
// In `VS Code` the configuration for this is `husky-lang-server.assist.importGranularity`.
//
// .Import Prefix
//
// The style of imports in the same crate is configurable through the `importPrefix` setting.
// It has the following configurations:
//
// - `by_crate`: This setting will force paths to be always absolute, starting with the `crate`
//  prefix, unless the item is defined outside of the current crate.
// - `by_self`: This setting will force paths that are relative to the current module to always
//  start with `self`. This will result in paths that always start with either `crate`, `self`,
//  `super` or an extern crate identifier.
// - `plain`: This setting does not impose any restrictions in imports.
//
// In `VS Code` the configuration for this is `husky-lang-server.assist.importPrefix`.
//
// image::https://user-images.githubusercontent.com/48062697/113020673-b85be580-917a-11eb-9022-59585f35d4f8.gif[]

// Assist: auto_import
//
// If the name is unresolved, provides all possible imports for it.
//
// ```
// fn main() {
//     let map = HashMap$0::new();
// }
// # pub mod std { pub mod collections { pub struct HashMap { } } }
// ```
// ->
// ```
// use std::collections::HashMap;
//
// fn main() {
//     let map = HashMap::new();
// }
// # pub mod std { pub mod collections { pub struct HashMap { } } }
// ```
pub(crate) fn auto_import(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

pub(super) fn find_importable_node(ctx: &AssistContext) -> Option<(ImportAssets, SyntaxElement)> {
    todo!()
}

fn group_label(import_candidate: &ImportCandidate) -> GroupLabel {
    let name = match import_candidate {
        ImportCandidate::Path(candidate) => format!("Import {}", candidate.name.text()),
        ImportCandidate::TraitAssocItem(candidate) => {
            format!(
                "Import a trait for item {}",
                candidate.assoc_item_name.text()
            )
        }
        ImportCandidate::TraitMethod(candidate) => {
            format!(
                "Import a trait for method {}",
                candidate.assoc_item_name.text()
            )
        }
    };
    GroupLabel(name)
}
