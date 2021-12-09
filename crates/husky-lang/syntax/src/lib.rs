#![allow(dead_code, unused)]
use std::{marker::PhantomData, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResult<T> {
    _ty: PhantomData<fn() -> T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SingleFileParseTree {}

pub mod ast;
pub struct SyntaxElementChildren {}
pub struct SyntaxToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxKind {}
pub struct SyntaxElement {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxNode {}
pub struct SyntaxNodePtr {}
pub mod token;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenNode {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Next,
    Prev,
}
pub struct TokenAtOffset<T> {
    phantom: std::marker::PhantomData<T>,
}
pub struct NodeOrToken<S, T> {
    phantoms: std::marker::PhantomData<S>,
    phantomt: std::marker::PhantomData<T>,
}

#[cfg(test)]
mod tests;
