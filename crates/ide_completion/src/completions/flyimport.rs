//! See [`import_on_the_fly`].
use ide_db::helpers::{
    import_assets::{ImportAssets, ImportCandidate},
    insert_use::ImportScope,
};
use itertools::Itertools;
use syntax::SyntaxNode;

use crate::{
    context::CompletionContext,
    render::{render_resolution_with_import, RenderContext},
    ImportEdit,
};

use super::Completions;

// Feature: Completion With Autoimport
//
// When completing names in the current scope, proposes additional imports from other modules or crates,
// if they can be qualified in the scope, and their name contains all symbols from the completion input.
//
// To be considered applicable, the name must contain all input symbols in the given order, not necessarily adjacent.
// If any input symbol is not lowercased, the name must contain all symbols in exact case; otherwise the containing is checked case-insensitively.
//
// ```
// fn main() {
//     pda$0
// }
// # pub mod std { pub mod marker { pub struct PhantomData { } } }
// ```
// ->
// ```
// use std::marker::PhantomData;
//
// fn main() {
//     PhantomData
// }
// # pub mod std { pub mod marker { pub struct PhantomData { } } }
// ```
//
// Also completes associated items, that require trait imports.
// If any unresolved and/or partially-qualified path precedes the input, it will be taken into account.
// Currently, only the imports with their import path ending with the whole qualifier will be proposed
// (no fuzzy matching for qualifier).
//
// ```
// mod foo {
//     pub mod bar {
//         pub struct Item;
//
//         impl Item {
//             pub const TEST_ASSOC: usize = 3;
//         }
//     }
// }
//
// fn main() {
//     bar::Item::TEST_A$0
// }
// ```
// ->
// ```
// use foo::bar;
//
// mod foo {
//     pub mod bar {
//         pub struct Item;
//
//         impl Item {
//             pub const TEST_ASSOC: usize = 3;
//         }
//     }
// }
//
// fn main() {
//     bar::Item::TEST_ASSOC
// }
// ```
//
// NOTE: currently, if an assoc item comes from a trait that's not currently imported, and it also has an unresolved and/or partially-qualified path,
// no imports will be proposed.
//
// .Fuzzy search details
//
// To avoid an excessive amount of the results returned, completion input is checked for inclusion in the names only
// (i.e. in `HashMap` in the `std::collections::HashMap` path).
// For the same reasons, avoids searching for any path imports for inputs with their length less than 2 symbols
// (but shows all associated items for any input length).
//
// .Import configuration
//
// It is possible to configure how use-trees are merged with the `importMergeBehavior` setting.
// Mimics the corresponding behavior of the `Auto Import` feature.
//
// .LSP and performance implications
//
// The feature is enabled only if the LSP client supports LSP protocol version 3.16+ and reports the `additionalTextEdits`
// (case-sensitive) resolve client capability in its client capabilities.
// This way the server is able to defer the costly computations, doing them for a selected completion item only.
// For clients with no such support, all edits have to be calculated on the completion request, including the fuzzy search completion ones,
// which might be slow ergo the feature is automatically disabled.
//
// .Feature toggle
//
// The feature can be forcefully turned off in the settings with the `husky-lang-server.completion.autoimport.enable` flag.
// Note that having this flag set to `true` does not guarantee that the feature is enabled: your client needs to have the corresponding
// capability enabled.
pub(crate) fn import_on_the_fly(acc: &mut Completions, ctx: &CompletionContext) -> Option<()> {
    todo!()
}

pub(crate) fn position_for_import(
    ctx: &CompletionContext,
    import_candidate: Option<&ImportCandidate>,
) -> Option<SyntaxNode> {
    todo!()
}

fn import_assets(ctx: &CompletionContext, fuzzy_name: String) -> Option<ImportAssets> {
    todo!()
}

pub(crate) fn compute_fuzzy_completion_order_key(
    proposed_mod_path: &hir::ModPath,
    user_input_lowercased: &str,
) -> usize {
    todo!()
}
