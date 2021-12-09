mod ast;
pub mod phrase;
pub mod sentence;
mod syntax_node;
pub mod token;

// extern crate common;

pub use phrase::*;
pub use rowan::{
    Direction, GreenNode, NodeOrToken, SyntaxText, TextRange, TextSize, TokenAtOffset, WalkEvent,
};
pub use sentence::*;
pub use token::*;

use std::{marker::PhantomData, sync::Arc};

use common::{FileTree, ParserError, Session, SymbolID};

pub struct AST {
    pub phrase_arena: PhraseArena,
    pub key: SymbolID,
    pub sentences: Vec<Sentence>,
    pub children: Vec<AST>,
}
impl AST {
    pub fn get_child(&self, key: SymbolID) -> &AST {
        self.children
            .iter()
            .find(|child| child.key == key)
            .expect("child should exist")
    }
    pub fn new(sources: &FileTree, sess: &mut Session) -> Result<AST, ParserError> {
        let mut ast = AST {
            phrase_arena: PhraseArena::new(),
            key: sess.get_symbol_id(&sources.key),
            sentences: vec![],
            children: vec![],
        };
        ast.parse(sources, sess)?;
        for source in &sources.children {
            ast.children.push(Self::new(source, sess)?)
        }
        Ok(ast)
    }
    pub fn parse(&mut self, sources: &FileTree, sess: &mut Session) -> Result<(), ParserError> {
        // let mut ast = AST {

        //   key: sources.key.clone(),
        self.sentences = parse_sentences(&sources.source, sess, &mut self.phrase_arena)?;
        // for source in &self.sources.children {
        //   ast
        //     .children
        //     .push(Self::parse(self.sess, source)?)
        // }
        Ok(())
    }
}

/// Represents the result of unsuccessful tokenization, parsing
/// or tree validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxError(String, TextRange);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseResult<T> {
    green: GreenNode,
    errors: Arc<Vec<SyntaxError>>,
    _ty: PhantomData<fn() -> T>,
}
