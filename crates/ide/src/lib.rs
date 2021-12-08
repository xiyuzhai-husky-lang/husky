//! ide crate provides "ide-centric" APIs for the husky-lang-server. That is,
//! it generally operates with files and text ranges, and returns results as
//! Strings, suitable for displaying to the human.
//!
//! What powers this API are the `RootDatabase` struct, which defines a `salsa`
//! database, and the `hir` crate, where majority of the analysis happens.
//! However, IDE specific bits of the analysis (most notably completion) happen
//! in this crate.

// For proving that RootDatabase is RefUnwindSafe.
#![recursion_limit = "128"]
#![allow(dead_code, unused)]
#[allow(unused)]
macro_rules! eprintln {
    ($($tt:tt)*) => { stdx::eprintln!($($tt)*) };
}

mod markup;
mod navigation_target;
mod prime_caches;

mod annotations;
mod call_hierarchy;
mod call_info;
mod doc_links;
mod extend_selection;
mod file_structure;
mod fn_references;
mod folding_ranges;
mod goto_declaration;
mod goto_definition;
mod goto_implementation;
mod goto_type_definition;
mod highlight_related;
mod hover;
mod join_lines;
mod markdown_remove;
mod matching_brace;
mod move_item;
mod parent_module;
mod references;
mod rename;
mod ssr;
mod static_index;
mod status;
mod syntax_highlighting;
mod syntax_tree;
mod typing;
mod view_hir;
mod view_item_tree;

use salsa::plumbing::SalsaInternalOpns;
use std::sync::Arc;

use common::*;

use husky_lang_db::{
    symbol_index::{self, FileSymbol},
    vfs::VirtualFileSystem,
    LineIndexDatabase,
};
use syntax::SingleFileParseTree;

use hir::db::DiagDatabase;

use crate::navigation_target::{ToNav, TryToNav};

pub use crate::{
    annotations::{Annotation, AnnotationConfig, AnnotationKind},
    call_hierarchy::CallItem,
    call_info::CallInfo,
    file_structure::{StructureNode, StructureNodeKind},
    folding_ranges::{Fold, FoldKind},
    highlight_related::{HighlightRelatedConfig, HighlightedRange},
    hover::{HoverAction, HoverConfig, HoverDocFormat, HoverGotoTypeData, HoverResult},
    join_lines::JoinLinesConfig,
    markup::Markup,
    move_item::Direction,
    navigation_target::NavigationTarget,
    prime_caches::PrimeCachesProgress,
    references::ReferenceSearchResult,
    rename::RenameError,
    static_index::{StaticIndex, StaticIndexedFile, TokenId, TokenStaticData},
    syntax_highlighting::{
        tags::{Highlight, HlMod, HlMods, HlOperator, HlPunct, HlTag},
        HlRange,
    },
};
pub use hir::{Documentation, Semantics};
pub use husky_lang_db::{
    label::Label,
    search::{ReferenceCategory, SearchScope},
    source_change::{FileSystemEdit, SourceChange},
    symbol_index::Query,
    vfs::{SourceFilePosition, SourceFileId, SourceFileRange},
    HuskyLangDatabase, SymbolKind,
};
pub use ide_assists::{
    Assist, AssistConfig, AssistId, AssistKind, AssistResolveStrategy, SingleResolve,
};
pub use ide_completion::{
    CompletionConfig, CompletionItem, CompletionItemKind, CompletionRelevance, ImportEdit, Snippet,
    SnippetScope,
};
pub use ide_ssr::SsrError;
pub use text_edit::{Indel, TextEdit};
use vfs::LineMap;

/// Info associated with a text range.
#[derive(Debug)]
pub struct RangeInfo<T> {
    pub range: TextRange,
    pub info: T,
}

impl<T> RangeInfo<T> {
    pub fn new(range: TextRange, info: T) -> RangeInfo<T> {
        RangeInfo { range, info }
    }
}

/// `AnalysisHost` stores the current state of the world.
#[derive(Debug)]
pub struct IdeDatabaseProxy {
    db: HuskyLangDatabase,
}

impl IdeDatabaseProxy {
    pub fn new(lru_capacity: Option<usize>) -> IdeDatabaseProxy {
        IdeDatabaseProxy {
            db: HuskyLangDatabase::new(lru_capacity),
        }
    }

    pub fn update_lru_capacity(&mut self, lru_capacity: Option<usize>) {
        self.db.update_lru_capacity(lru_capacity);
    }

    /// Returns a snapshot of the current state, which you can query for
    /// semantic information.
    pub fn snapshot(&self) -> IdeDatabaseSnapshot {
        todo!() // IdeDatabaseSnapshot {
                //     db: self.db.snapshot(),
                // }
    }

    /// NB: this clears the database
    pub fn per_query_memory_usage(&mut self) -> Vec<(String, profile::Bytes)> {
        todo!()
    }
    pub fn request_cancellation(&mut self) {
        self.db.request_cancellation();
    }
    pub fn raw_database(&self) -> &HuskyLangDatabase {
        &self.db
    }
    pub fn raw_database_mut(&mut self) -> &mut HuskyLangDatabase {
        &mut self.db
    }

    pub fn path(&self, file_id: SourceFileId) -> &Path {
        todo!()
    }
    pub fn set_file_content(&mut self, path: vfs::VirtualPath, content: Option<Vec<u8>>) {
        todo!()
    }

    pub fn get_file_line_collection(&self, file_id: vfs::SourceFileId) -> Result<LineMap> {
        todo!()
    }
}

impl Default for IdeDatabaseProxy {
    fn default() -> IdeDatabaseProxy {
        IdeDatabaseProxy::new(None)
    }
}

pub struct Cancellable<T> {
    phantom: std::marker::PhantomData<T>,
}

/// Analysis is a snapshot of a world state at a moment in time. It is the main
/// entry point for asking semantic information about the world. When the world
/// state is advanced using `AnalysisHost::apply_change` method, all existing
/// `Analysis` are canceled (most method return `Err(Canceled)`).
#[derive(Debug)]
pub struct IdeDatabaseSnapshot {
    db: salsa::Snapshot<HuskyLangDatabase>,
}

// As a general design guideline, `Analysis` API are intended to be independent
// from the language server protocol. That is, when exposing some functionality
// we should think in terms of "what API makes most sense" and not in terms of
// "what types LSP uses". Although currently LSP is the only consumer of the
// API, the API should in theory be usable as a library, or via a different
// protocol.
impl IdeDatabaseSnapshot {
    // Creates an analysis instance for a single file, without any extenal
    // dependencies, stdlib support or ability to apply changes. See
    // `AnalysisHost` for creating a fully-featured analysis.
    pub fn from_single_file(text: String) -> (IdeDatabaseSnapshot, SourceFileId) {
        todo!()
    }

    /// Debug info about the current state of the analysis.
    pub fn status(&self, file_id: Option<SourceFileId>) -> Cancellable<String> {
        self.try_db_query(|db| status::status(&*db, file_id))
    }

    pub fn prime_caches<F>(&self, cb: F) -> Cancellable<()>
    where
        F: Fn(PrimeCachesProgress) + Sync + std::panic::UnwindSafe,
    {
        self.try_db_query(move |db| prime_caches::prime_caches(db, &cb))
    }

    /// Gets the text of the source file.
    pub fn file_text(&self, file_id: SourceFileId) -> Cancellable<Arc<String>> {
        todo!()
        // self.try_db_query(|db| db.file_text(file_id))
    }

    /// Gets the syntax tree of the file.
    pub fn parse(&self, file_id: SourceFileId) -> Cancellable<SingleFileParseTree> {
        todo!()
    }

    /// Returns true if this file belongs to an immutable library.
    pub fn is_library_file(&self, file_id: SourceFileId) -> Cancellable<bool> {
        todo!();
        // use husky_lang_db::vfs::SourceDatabaseExt;
        // self.try_db_query(|db| db.source_root(db.package_root(file_id)).is_library)
    }

    /// Gets the file's `LineIndex`: data structure to convert between absolute
    /// offsets and line/column representation.
    pub fn file_line_index(&self, file_id: SourceFileId) -> Cancellable<Arc<LineMap>> {
        todo!()
        // self.try_db_query(|db| db.line_index(file_id))
    }

    /// Selects the next syntactic nodes encompassing the range.
    pub fn extend_selection(&self, frange: SourceFileRange) -> Cancellable<TextRange> {
        self.try_db_query(|db| extend_selection::extend_selection(db, frange))
    }

    /// Returns position of the matching brace (all types of braces are
    /// supported).
    pub fn matching_brace(&self, position: SourceFilePosition) -> Cancellable<Option<TextSize>> {
        todo!()
    }

    /// Returns a syntax tree represented as `String`, for debug purposes.
    // FIXME: use a better name here.
    pub fn syntax_tree(
        &self,
        file_id: SourceFileId,
        text_range: Option<TextRange>,
    ) -> Cancellable<String> {
        self.try_db_query(|db| syntax_tree::syntax_tree(db, file_id, text_range))
    }

    pub fn view_hir(&self, position: SourceFilePosition) -> Cancellable<String> {
        self.try_db_query(|db| view_hir::view_hir(db, position))
    }

    pub fn view_item_tree(&self, file_id: SourceFileId) -> Cancellable<String> {
        self.try_db_query(|db| view_item_tree::view_item_tree(db, file_id))
    }

    /// Returns an edit to remove all newlines in the range, cleaning up minor
    /// stuff like trailing commas.
    pub fn join_lines(
        &self,
        config: &JoinLinesConfig,
        frange: SourceFileRange,
    ) -> Cancellable<TextEdit> {
        todo!()
    }

    /// Returns an edit which should be applied when opening a new line, fixing
    /// up minor stuff like continuing the comment.
    /// The edit will be a snippet (with `$0`).
    pub fn on_enter(&self, position: SourceFilePosition) -> Cancellable<Option<TextEdit>> {
        self.try_db_query(|db| typing::on_enter(db, position))
    }

    /// Returns an edit which should be applied after a character was typed.
    ///
    /// This is useful for some on-the-fly fixups, like adding `;` to `let =`
    /// automatically.
    pub fn on_char_typed(
        &self,
        position: SourceFilePosition,
        char_typed: char,
    ) -> Cancellable<Option<SourceChange>> {
        todo!()
        // // Fast path to not even parse the file.
        // if !typing::TRIGGER_CHARS.contains(char_typed) {
        //     return Ok(None);
        // }
        // self.try_db_query(|db| typing::on_char_typed(db, position, char_typed))
    }

    /// Returns a tree representation of symbols in the file. Useful to draw a
    /// file outline.
    pub fn file_structure(&self, file_id: SourceFileId) -> Cancellable<Vec<StructureNode>> {
        todo!()
        // eprintln!("TODO: tree representation of symbols in the file");
        // Ok(vec![])
    }

    /// Returns the set of folding ranges.
    pub fn folding_ranges(&self, file_id: SourceFileId) -> Cancellable<Vec<Fold>> {
        todo!()
        // eprintln!("TODO: folding_ranges");
        // Ok(vec![])
    }

    /// Fuzzy searches for a symbol.
    pub fn symbol_search(&self, query: Query) -> Cancellable<Vec<NavigationTarget>> {
        self.try_db_query(|db| {
            symbol_index::world_symbols(db, query)
                .into_iter()
                .map(|s| s.to_nav(db))
                .collect::<Vec<_>>()
        })
    }

    /// Returns the definitions from the symbol at `position`.
    pub fn goto_definition(
        &self,
        position: SourceFilePosition,
    ) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>> {
        self.try_db_query(|db| goto_definition::goto_definition(db, position))
    }

    /// Returns the declaration from the symbol at `position`.
    pub fn goto_declaration(
        &self,
        position: SourceFilePosition,
    ) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>> {
        self.try_db_query(|db| goto_declaration::goto_declaration(db, position))
    }

    /// Returns the impls from the symbol at `position`.
    pub fn goto_implementation(
        &self,
        position: SourceFilePosition,
    ) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>> {
        self.try_db_query(|db| goto_implementation::goto_implementation(db, position))
    }

    /// Returns the type definitions for the symbol at `position`.
    pub fn goto_type_definition(
        &self,
        position: SourceFilePosition,
    ) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>> {
        self.try_db_query(|db| goto_type_definition::goto_type_definition(db, position))
    }

    /// Finds all usages of the reference at point.
    pub fn find_all_refs(
        &self,
        position: SourceFilePosition,
        search_scope: Option<SearchScope>,
    ) -> Cancellable<Option<Vec<ReferenceSearchResult>>> {
        todo!()
    }

    /// Finds all methods and free functions for the file. Does not return tests!
    pub fn find_all_methods(&self, file_id: SourceFileId) -> Cancellable<Vec<SourceFileRange>> {
        self.try_db_query(|db| fn_references::find_all_methods(db, file_id))
    }

    /// Returns a short text describing element at position.
    pub fn hover(
        &self,
        config: &HoverConfig,
        range: SourceFileRange,
    ) -> Cancellable<Option<RangeInfo<HoverResult>>> {
        self.try_db_query(|db| hover::hover(db, range, config))
    }

    /// Return URL(s) for the documentation of the symbol under the cursor.
    pub fn external_docs(
        &self,
        position: SourceFilePosition,
    ) -> Cancellable<Option<doc_links::DocumentationLink>> {
        self.try_db_query(|db| doc_links::external_docs(db, &position))
    }

    /// Computes parameter information for the given call expression.
    pub fn call_info(&self, position: SourceFilePosition) -> Cancellable<Option<CallInfo>> {
        self.try_db_query(|db| call_info::call_info(db, position))
    }

    /// Computes call hierarchy candidates for the given file position.
    pub fn call_hierarchy(
        &self,
        position: SourceFilePosition,
    ) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>> {
        self.try_db_query(|db| call_hierarchy::call_hierarchy(db, position))
    }

    /// Computes incoming calls for the given file position.
    pub fn incoming_calls(&self, position: SourceFilePosition) -> Cancellable<Option<Vec<CallItem>>> {
        self.try_db_query(|db| call_hierarchy::incoming_calls(db, position))
    }

    /// Computes outgoing calls for the given file position.
    pub fn outgoing_calls(&self, position: SourceFilePosition) -> Cancellable<Option<Vec<CallItem>>> {
        self.try_db_query(|db| call_hierarchy::outgoing_calls(db, position))
    }

    /// Returns a `mod name;` declaration which created the current module.
    pub fn parent_module(&self, position: SourceFilePosition) -> Cancellable<Vec<NavigationTarget>> {
        self.try_db_query(|db| parent_module::parent_module(db, position))
    }

    /// Computes syntax highlighting for the given file
    pub fn highlight(&self, file_id: SourceFileId) -> Cancellable<Vec<HlRange>> {
        self.try_db_query(|db| syntax_highlighting::highlight(db, file_id, None, false))
    }

    /// Computes all ranges to highlight for a given item in a file.
    pub fn highlight_related(
        &self,
        config: HighlightRelatedConfig,
        position: SourceFilePosition,
    ) -> Cancellable<Option<Vec<HighlightedRange>>> {
        todo!()
        // eprintln!("TODO: all ranges to highlight for a given item in a file");
        // Ok(None)
    }

    /// Computes syntax highlighting for the given file range.
    pub fn highlight_range(&self, frange: SourceFileRange) -> Cancellable<Vec<HlRange>> {
        self.try_db_query(|db| {
            syntax_highlighting::highlight(db, frange.file_id, Some(frange.range), false)
        })
    }

    /// Computes syntax highlighting for the given file.
    pub fn highlight_as_html(&self, file_id: SourceFileId, rainbow: bool) -> Cancellable<String> {
        todo!()
    }

    /// Computes completions at the given position.
    pub fn completions(
        &self,
        config: &CompletionConfig,
        position: SourceFilePosition,
    ) -> Cancellable<Option<Vec<CompletionItem>>> {
        self.try_db_query(|db| ide_completion::completions(db, config, position).map(Into::into))
    }

    /// Resolves additional completion data at the position given.
    pub fn resolve_completion_edits(
        &self,
        config: &CompletionConfig,
        position: SourceFilePosition,
        imports: impl IntoIterator<Item = (String, String)> + std::panic::UnwindSafe,
    ) -> Cancellable<Vec<TextEdit>> {
        todo!()
        // Ok(self
        //     .try_db_query(|db| {
        //         ide_completion::resolve_completion_edits(db, config, position, imports)
        //     })?
        //     .unwrap_or_default())
    }

    /// Computes assists (aka code actions aka intentions) for the given
    /// position. If `resolve == false`, computes enough info to show the
    /// lightbulb list in the editor, but doesn't compute actual edits, to
    /// improve performance.
    pub fn assists(
        &self,
        config: &AssistConfig,
        resolve: AssistResolveStrategy,
        frange: SourceFileRange,
    ) -> Cancellable<Vec<Assist>> {
        self.try_db_query(|db| {
            let ssr_assists = ssr::ssr_assists(db, &resolve, frange);
            let mut acc = ide_assists::assists(db, config, resolve, frange);
            acc.extend(ssr_assists.into_iter());
            acc
        })
    }

    /// Computes the set of diagnostics for the given file.
    pub fn diagnostics(&self, file_id: SourceFileId) -> Cancellable<Vec<hir::Diagnostic>> {
        todo!()
        // self.try_db_query(|db| db.diagnostics(file_id))
    }

    /// Returns the edit required to rename reference at the position to the new
    /// name.
    pub fn rename(
        &self,
        position: SourceFilePosition,
        new_name: &str,
    ) -> Cancellable<Result<SourceChange, RenameError>> {
        self.try_db_query(|db| rename::rename(db, position, new_name))
    }

    pub fn prepare_rename(
        &self,
        position: SourceFilePosition,
    ) -> Cancellable<Result<RangeInfo<()>, RenameError>> {
        self.try_db_query(|db| rename::prepare_rename(db, position))
    }

    pub fn will_rename_file(
        &self,
        file_id: SourceFileId,
        new_name_stem: &str,
    ) -> Cancellable<Option<SourceChange>> {
        self.try_db_query(|db| rename::will_rename_file(db, file_id, new_name_stem))
    }

    pub fn structural_search_replace(
        &self,
        query: &str,
        parse_only: bool,
        resolve_context: SourceFilePosition,
        selections: Vec<SourceFileRange>,
    ) -> Cancellable<Result<SourceChange, SsrError>> {
        self.try_db_query(|db| {
            let rule: ide_ssr::SsrRule = query.parse()?;
            let mut match_finder =
                ide_ssr::MatchFinder::in_context(db, resolve_context, selections);
            match_finder.add_rule(rule)?;
            let edits = if parse_only {
                Default::default()
            } else {
                match_finder.edits()
            };
            Ok(SourceChange::from(edits))
        })
    }

    pub fn annotations(
        &self,
        config: &AnnotationConfig,
        file_id: SourceFileId,
    ) -> Cancellable<Vec<Annotation>> {
        self.try_db_query(|db| annotations::annotations(db, config, file_id))
    }

    pub fn resolve_annotation(&self, annotation: Annotation) -> Cancellable<Annotation> {
        self.try_db_query(|db| annotations::resolve_annotation(db, annotation))
    }

    pub fn move_item(
        &self,
        range: SourceFileRange,
        direction: Direction,
    ) -> Cancellable<Option<TextEdit>> {
        self.try_db_query(|db| move_item::move_item(db, range, direction))
    }

    /// Performs an operation on the database that may be canceled.
    ///
    /// husky-lang-server needs to be able to answer semantic questions about the
    /// code while the code is being modified. A common problem is that a
    /// long-running query is being calculated when a new change arrives.
    ///
    /// We can't just apply the change immediately: this will cause the pending
    /// query to see inconsistent state (it will observe an absence of
    /// repeatable read). So what we do is we **cancel** all pending queries
    /// before applying the change.
    ///
    /// Salsa implements cancelation by unwinding with a special value and
    /// catching it on the API boundary.
    fn try_db_query<F, T>(&self, f: F) -> Cancellable<T>
    where
        F: FnOnce(&HuskyLangDatabase) -> T + std::panic::UnwindSafe,
    {
        todo!()
        // Cancelled::catch(|| f(&self.db))
    }
}
