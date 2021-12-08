use common::*;

use husky_lang_db::SymbolKind;
use syntax::{ast, NodeOrToken, SingleFileParseTree, SyntaxNode, SyntaxToken};

#[derive(Debug, Clone)]
pub struct StructureNode {
    pub parent: Option<usize>,
    pub label: String,
    pub navigation_range: TextRange,
    pub node_range: TextRange,
    pub kind: StructureNodeKind,
    pub detail: Option<String>,
    pub deprecated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StructureNodeKind {
    SymbolKind(SymbolKind),
    Region,
}

// Feature: File Structure
//
// Provides a tree of the symbols defined in the file. Can be used to
//
// * fuzzy search symbol in a file (super useful)
// * draw breadcrumbs to describe the context around the cursor
// * draw outline of the file
//
// |===
// | Editor  | Shortcut
//
// | VS Code | kbd:[Ctrl+Shift+O]
// |===
//
// image::https://user-images.githubusercontent.com/48062697/113020654-b42fc800-917a-11eb-8388-e7dc4d92b02e.gif[]

pub(crate) fn file_structure(file: &SingleFileParseTree) -> Vec<StructureNode> {
    todo!()
}

fn structure_node(node: &SyntaxNode) -> Option<StructureNode> {
    todo!()
}

fn structure_token(token: SyntaxToken) -> Option<StructureNode> {
    todo!()
}
