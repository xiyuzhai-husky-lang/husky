use syntax::{ast, SingleFileParseTree, SyntaxKind, TextSize};

// Feature: Matching Brace
//
// If the cursor is on any brace (`<>(){}[]||`) which is a part of a brace-pair,
// moves cursor to the matching brace. It uses the actual parser to determine
// braces, so it won't confuse generics with comparisons.
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Rust Analyzer: Find matching brace**
// |===
//
// image::https://user-images.githubusercontent.com/48062697/113065573-04298180-91b1-11eb-8dec-d4e2a202f304.gif[]
pub(crate) fn matching_brace(file: &SingleFileParseTree, offset: TextSize) -> Option<TextSize> {
    todo!()
}
