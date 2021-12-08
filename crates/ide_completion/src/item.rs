//! See `CompletionItem` structure.

use std::fmt;

use common::*;

use hir::{Documentation, Mutability};
use husky_lang_db::{
    helpers::{
        import_assets::LocatedImport,
        insert_use::{self, ImportScope, InsertUseConfig},
        mod_path_to_ast, SnippetCap,
    },
    SymbolKind,
};
use smallvec::SmallVec;
use stdx::{impl_from, never};
use syntax::SmolStr;
use text_edit::TextEdit;

/// `CompletionItem` describes a single completion variant in the editor pop-up.
/// It is basically a POD with various properties. To construct a
/// `CompletionItem`, use `new` method and the `Builder` struct.
#[derive(Clone)]
pub struct CompletionItem {
    /// Label in the completion pop up which identifies completion.
    label: SmolStr,
    /// Range of identifier that is being completed.
    ///
    /// It should be used primarily for UI, but we also use this to convert
    /// generic TextEdit into LSP's completion edit (see conv.rs).
    ///
    /// `source_range` must contain the completion offset. `insert_text` should
    /// start with what `source_range` points to, or VSCode will filter out the
    /// completion silently.
    source_range: TextRange,
    /// What happens when user selects this item.
    ///
    /// Typically, replaces `source_range` with new identifier.
    text_edit: TextEdit,
    is_snippet: bool,

    /// What item (struct, function, etc) are we completing.
    kind: CompletionItemKind,

    /// Lookup is used to check if completion item indeed can complete current
    /// ident.
    ///
    /// That is, in `foo.bar$0` lookup of `abracadabra` will be accepted (it
    /// contains `bar` sub sequence), and `quux` will rejected.
    lookup: Option<SmolStr>,

    /// Additional info to show in the UI pop up.
    detail: Option<String>,
    documentation: Option<Documentation>,

    /// Whether this item is marked as deprecated
    deprecated: bool,

    /// If completing a function call, ask the editor to show parameter popup
    /// after completion.
    trigger_call_info: bool,

    /// We use this to sort completion. Relevance records facts like "do the
    /// types align precisely?". We can't sort by relevances directly, they are
    /// only partially ordered.
    ///
    /// Note that Relevance ignores fuzzy match score. We compute Relevance for
    /// all possible items, and then separately build an ordered completion list
    /// based on relevance and fuzzy matching with the already typed identifier.
    relevance: CompletionRelevance,

    /// Indicates that a reference or mutable reference to this variable is a
    /// possible match.
    ref_match: Option<Mutability>,

    /// The import data to add to completion's edits.
    import_to_add: SmallVec<[ImportEdit; 1]>,
}

// We use custom debug for CompletionItem to make snapshot tests more readable.
impl fmt::Debug for CompletionItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct CompletionRelevance {
    /// This is set in cases like these:
    ///
    /// ```
    /// fn f(spam: String) {}
    /// fn main {
    ///     let spam = 92;
    ///     f($0) // name of local matches the name of param
    /// }
    /// ```
    pub exact_name_match: bool,
    /// See CompletionRelevanceTypeMatch doc comments for cases where this is set.
    pub type_match: Option<CompletionRelevanceTypeMatch>,
    /// This is set in cases like these:
    ///
    /// ```
    /// fn foo(a: u32) {
    ///     let b = 0;
    ///     $0 // `a` and `b` are local
    /// }
    /// ```
    pub is_local: bool,
    /// This is set in cases like these:
    ///
    /// ```
    /// (a > b).not$0
    /// ```
    ///
    /// Basically, we want to guarantee that postfix snippets always takes
    /// precedence over everything else.
    pub exact_postfix_snippet_match: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompletionRelevanceTypeMatch {
    /// This is set in cases like these:
    ///
    /// ```
    /// enum Option<T> { Some(T), None }
    /// fn f(a: Option<u32>) {}
    /// fn main {
    ///     f(Option::N$0) // type `Option<T>` could unify with `Option<u32>`
    /// }
    /// ```
    CouldUnify,
    /// This is set in cases like these:
    ///
    /// ```
    /// fn f(spam: String) {}
    /// fn main {
    ///     let foo = String::new();
    ///     f($0) // type of local matches the type of param
    /// }
    /// ```
    Exact,
}

impl CompletionRelevance {
    /// Provides a relevance score. Higher values are more relevant.
    ///
    /// The absolute value of the relevance score is not meaningful, for
    /// example a value of 0 doesn't mean "not relevant", rather
    /// it means "least relevant". The score value should only be used
    /// for relative ordering.
    ///
    /// See is_relevant if you need to make some judgement about score
    /// in an absolute sense.
    pub fn score(&self) -> u32 {
        let mut score = 0;

        if self.exact_name_match {
            score += 1;
        }
        score += match self.type_match {
            Some(CompletionRelevanceTypeMatch::Exact) => 4,
            Some(CompletionRelevanceTypeMatch::CouldUnify) => 3,
            None => 0,
        };
        if self.is_local {
            score += 1;
        }
        if self.exact_postfix_snippet_match {
            score += 100;
        }
        score
    }

    /// Returns true when the score for this threshold is above
    /// some threshold such that we think it is especially likely
    /// to be relevant.
    pub fn is_relevant(&self) -> bool {
        self.score() > 0
    }
}

/// The type of the completion item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionItemKind {
    SymbolKind(SymbolKind),
    Attribute,
    Binding,
    BuiltinType,
    Keyword,
    Method,
    Snippet,
    UnresolvedReference,
}

impl_from!(SymbolKind for CompletionItemKind);

impl CompletionItem {
    pub(crate) fn new(
        kind: impl Into<CompletionItemKind>,
        source_range: TextRange,
        label: impl Into<SmolStr>,
    ) -> Builder {
        let label = label.into();
        Builder {
            source_range,
            label,
            insert_text: None,
            is_snippet: false,
            trait_name: None,
            detail: None,
            documentation: None,
            lookup: None,
            kind: kind.into(),
            text_edit: None,
            deprecated: false,
            trigger_call_info: None,
            relevance: CompletionRelevance::default(),
            ref_match: None,
            imports_to_add: Default::default(),
        }
    }

    /// What user sees in pop-up in the UI.
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn source_range(&self) -> TextRange {
        self.source_range
    }

    pub fn text_edit(&self) -> &TextEdit {
        &self.text_edit
    }
    /// Whether `text_edit` is a snippet (contains `$0` markers).
    pub fn is_snippet(&self) -> bool {
        self.is_snippet
    }

    /// Short one-line additional information, like a type
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }
    /// A doc-comment
    pub fn documentation(&self) -> Option<Documentation> {
        self.documentation.clone()
    }
    /// What string is used for filtering.
    pub fn lookup(&self) -> &str {
        self.lookup.as_deref().unwrap_or(&self.label)
    }

    pub fn kind(&self) -> CompletionItemKind {
        self.kind
    }

    pub fn deprecated(&self) -> bool {
        self.deprecated
    }

    pub fn relevance(&self) -> CompletionRelevance {
        self.relevance
    }

    pub fn trigger_call_info(&self) -> bool {
        self.trigger_call_info
    }

    pub fn ref_match(&self) -> Option<(Mutability, CompletionRelevance)> {
        todo!()
    }

    pub fn imports_to_add(&self) -> &[ImportEdit] {
        &self.import_to_add
    }
}

/// An extra import to add after the completion is applied.
#[derive(Debug, Clone)]
pub struct ImportEdit {
    pub import: LocatedImport,
    pub scope: ImportScope,
}

impl ImportEdit {
    /// Attempts to insert the import to the given scope, producing a text edit.
    /// May return no edit in edge cases, such as scope already containing the import.
    pub fn to_text_edit(&self, cfg: InsertUseConfig) -> Option<TextEdit> {
        todo!()
    }
}

/// A helper to make `CompletionItem`s.
#[must_use]
#[derive(Clone)]
pub(crate) struct Builder {
    source_range: TextRange,
    imports_to_add: SmallVec<[ImportEdit; 1]>,
    trait_name: Option<SmolStr>,
    label: SmolStr,
    insert_text: Option<String>,
    is_snippet: bool,
    detail: Option<String>,
    documentation: Option<Documentation>,
    lookup: Option<SmolStr>,
    kind: CompletionItemKind,
    text_edit: Option<TextEdit>,
    deprecated: bool,
    trigger_call_info: Option<bool>,
    relevance: CompletionRelevance,
    ref_match: Option<Mutability>,
}

impl Builder {
    pub(crate) fn build(self) -> CompletionItem {
        todo!()
    }
    pub(crate) fn lookup_by(&mut self, lookup: impl Into<SmolStr>) -> &mut Builder {
        self.lookup = Some(lookup.into());
        self
    }
    pub(crate) fn label(&mut self, label: impl Into<SmolStr>) -> &mut Builder {
        self.label = label.into();
        self
    }
    pub(crate) fn trait_name(&mut self, trait_name: SmolStr) -> &mut Builder {
        self.trait_name = Some(trait_name);
        self
    }
    pub(crate) fn insert_text(&mut self, insert_text: impl Into<String>) -> &mut Builder {
        self.insert_text = Some(insert_text.into());
        self
    }
    pub(crate) fn insert_snippet(
        &mut self,
        cap: SnippetCap,
        snippet: impl Into<String>,
    ) -> &mut Builder {
        let _ = cap;
        self.is_snippet = true;
        self.insert_text(snippet)
    }
    pub(crate) fn text_edit(&mut self, edit: TextEdit) -> &mut Builder {
        self.text_edit = Some(edit);
        self
    }
    pub(crate) fn snippet_edit(&mut self, _cap: SnippetCap, edit: TextEdit) -> &mut Builder {
        self.is_snippet = true;
        self.text_edit(edit)
    }
    pub(crate) fn detail(&mut self, detail: impl Into<String>) -> &mut Builder {
        self.set_detail(Some(detail))
    }
    pub(crate) fn set_detail(&mut self, detail: Option<impl Into<String>>) -> &mut Builder {
        self.detail = detail.map(Into::into);
        if let Some(detail) = &self.detail {
            if never!(detail.contains('\n'), "multiline detail:\n{}", detail) {
                self.detail = Some(detail.splitn(2, '\n').next().unwrap().to_string());
            }
        }
        self
    }
    #[allow(unused)]
    pub(crate) fn documentation(&mut self, docs: Documentation) -> &mut Builder {
        self.set_documentation(Some(docs))
    }
    pub(crate) fn set_documentation(&mut self, docs: Option<Documentation>) -> &mut Builder {
        self.documentation = docs.map(Into::into);
        self
    }
    pub(crate) fn set_deprecated(&mut self, deprecated: bool) -> &mut Builder {
        self.deprecated = deprecated;
        self
    }
    pub(crate) fn set_relevance(&mut self, relevance: CompletionRelevance) -> &mut Builder {
        self.relevance = relevance;
        self
    }
    pub(crate) fn trigger_call_info(&mut self) -> &mut Builder {
        self.trigger_call_info = Some(true);
        self
    }
    pub(crate) fn add_import(&mut self, import_to_add: ImportEdit) -> &mut Builder {
        self.imports_to_add.push(import_to_add);
        self
    }
    pub(crate) fn ref_match(&mut self, mutability: Mutability) -> &mut Builder {
        self.ref_match = Some(mutability);
        self
    }
}
