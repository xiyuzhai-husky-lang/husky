//! This module provides functionality for querying callable information about a token.

use either::Either;
use hir::{Semantics, Type};
use syntax::{ast, SyntaxToken};

use crate::IdeDatabase;

#[derive(Debug)]
pub struct ActiveParameter {
    pub ty: Type,
    pub pat: Either<ast::SelfParam, ast::Pat>,
}

impl ActiveParameter {
    /// Returns information about the call argument this token is part of.
    pub fn at_token(sema: &Semantics<IdeDatabase>, token: SyntaxToken) -> Option<Self> {
        todo!()
    }

    pub fn ident(&self) -> Option<ast::Name> {
        todo!()
    }
}

/// Returns a [`hir::Callable`] this token is a part of and its argument index of said callable.
pub fn callable_for_token(
    sema: &Semantics<IdeDatabase>,
    token: SyntaxToken,
) -> Option<(hir::Callable, Option<usize>)> {
    todo!()
}
