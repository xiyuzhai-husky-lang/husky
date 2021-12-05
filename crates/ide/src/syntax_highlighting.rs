pub(crate) mod tags;

mod highlights;
mod injector;

mod format;
mod highlight;
mod inject;

use hir::{InFile, Name, Semantics};
use ide_db::{RootDatabase, SymbolKind};
use rustc_hash::FxHashMap;
use syntax::{
    ast::{self, HasFormatSpecifier},
    Direction, NodeOrToken,
    SyntaxKind::*,
    SyntaxNode, TextRange, WalkEvent,
};

use crate::{
    syntax_highlighting::{
        format::highlight_format_string, highlights::Highlights, tags::Highlight,
    },
    FileID, HlMod, HlTag,
};

#[derive(Debug, Clone, Copy)]
pub struct HlRange {
    pub range: TextRange,
    pub highlight: Highlight,
    pub binding_hash: Option<u64>,
}

// Feature: Semantic Syntax Highlighting
//
// rust-analyzer highlights the code semantically.
// For example, `Bar` in `foo::Bar` might be colored differently depending on whether `Bar` is an enum or a trait.
// rust-analyzer does not specify colors directly, instead it assigns a tag (like `struct`) and a set of modifiers (like `declaration`) to each token.
// It's up to the client to map those to specific colors.
//
// The general rule is that a reference to an entity gets colored the same way as the entity itself.
// We also give special modifier for `mut` and `&mut` local variables.
//
//
// .Token Tags
//
// Rust-analyzer currently emits the following token tags:
//
// - For items:
// +
// [horizontal]
// enum:: Emitted for enums.
// function:: Emitted for free-standing functions.
// macro:: Emitted for macros.
// method:: Emitted for associated functions, also knowns as methods.
// namespace:: Emitted for modules.
// struct:: Emitted for structs.
// trait:: Emitted for traits.
// typeAlias:: Emitted for type aliases and `Self` in `impl`s.
// union:: Emitted for unions.
//
// - For literals:
// +
// [horizontal]
// boolean:: Emitted for the boolean literals `true` and `false`.
// character:: Emitted for character literals.
// number:: Emitted for numeric literals.
// string:: Emitted for string literals.
// escapeSequence:: Emitted for escaped sequences inside strings like `\n`.
// formatSpecifier:: Emitted for format specifiers `{:?}` in `format!`-like macros.
//
// - For operators:
// +
// [horizontal]
// operator:: Emitted for general operators.
// arithmetic:: Emitted for the arithmetic operators `+`, `-`, `*`, `/`, `+=`, `-=`, `*=`, `/=`.
// bitwise:: Emitted for the bitwise operators `|`, `&`, `!`, `^`, `|=`, `&=`, `^=`.
// comparison:: Emitted for the comparison operators `>`, `<`, `==`, `>=`, `<=`, `!=`.
// logical:: Emitted for the logical operators `||`, `&&`, `!`.
//
// - For punctuation:
// +
// [horizontal]
// punctuation:: Emitted for general punctuation.
// angle:: Emitted for `<>` angle brackets.
// brace:: Emitted for `{}` braces.
// bracket:: Emitted for `[]` brackets.
// parenthesis:: Emitted for `()` parentheses.
// colon:: Emitted for the `:` token.
// comma:: Emitted for the `,` token.
// dot:: Emitted for the `.` token.
// Semi:: Emitted for the `;` token.
//
// //-
//
// [horizontal]
// attribute:: Emitted for the `#[` `]` tokens.
// builtinAttribute:: Emitted for names to builtin attributes in attribute path, the `repr` in `#[repr(u8)]` for example.
// builtinType:: Emitted for builtin types like `u32`, `str` and `f32`.
// comment:: Emitted for comments.
// constParameter:: Emitted for const parameters.
// enumMember:: Emitted for enum variants.
// generic:: Emitted for generic tokens that have no mapping.
// keyword:: Emitted for keywords.
// label:: Emitted for labels.
// lifetime:: Emitted for lifetimes.
// parameter:: Emitted for non-self function parameters.
// property:: Emitted for struct and union fields.
// selfKeyword:: Emitted for the self function parameter and self path-specifier.
// typeParameter:: Emitted for type parameters.
// unresolvedReference:: Emitted for unresolved references, names that rust-analyzer can't find the definition of.
// variable:: Emitted for locals, constants and statics.
//
//
// .Token Modifiers
//
// Token modifiers allow to style some elements in the source code more precisely.
//
// Rust-analyzer currently emits the following token modifiers:
//
// [horizontal]
// async:: Emitted for async functions and the `async` and `await` keywords.
// attribute:: Emitted for tokens inside attributes.
// callable:: Emitted for locals whose types implements one of the `Fn*` traits.
// constant:: Emitted for consts.
// consuming:: Emitted for locals that are being consumed when use in a function call.
// controlFlow:: Emitted for control-flow related tokens, this includes the `?` operator.
// crateRoot:: Emitted for crate names, like `serde` and `crate`.
// declaration:: Emitted for names of definitions, like `foo` in `fn foo() {}`.
// defaultLibrary:: Emitted for items from built-in crates (std, core, alloc, test and proc_macro).
// documentation:: Emitted for documentation comments.
// injected:: Emitted for doc-string injected highlighting like rust source blocks in documentation.
// intraDocLink:: Emitted for intra doc links in doc-strings.
// library:: Emitted for items that are defined outside of the current crate.
// mutable:: Emitted for mutable locals and statics as well as functions taking `&mut self`.
// public:: Emitted for items that are from the current crate and are `pub`.
// reference:: Emitted for locals behind a reference and functions taking `self` by reference.
// static:: Emitted for "static" functions, also known as functions that do not take a `self` param, as well as statics and consts.
// trait:: Emitted for associated trait items.
// unsafe:: Emitted for unsafe operations, like unsafe function calls, as well as the `unsafe` token.
//
//
// image::https://user-images.githubusercontent.com/48062697/113164457-06cfb980-9239-11eb-819b-0f93e646acf8.png[]
// image::https://user-images.githubusercontent.com/48062697/113187625-f7f50100-9250-11eb-825e-91c58f236071.png[]
pub(crate) fn highlight(
    db: &RootDatabase,
    file_id: FileID,
    range_to_highlight: Option<TextRange>,
    syntactic_name_ref_highlighting: bool,
) -> Vec<HlRange> {
    todo!()
}

fn traverse(
    hl: &mut Highlights,
    sema: &Semantics<RootDatabase>,
    root: InFile<&SyntaxNode>,
    krate: Option<hir::Crate>,
    range_to_highlight: TextRange,
    syntactic_name_ref_highlighting: bool,
) {
    todo!()
}
