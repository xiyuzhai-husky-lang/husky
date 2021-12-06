//! This module contains structures for filtering the expected types.
//! Use case for structures in this module is, for example, situation when you need to process
//! only certain `Enum`s.

use std::iter;

use hir::Semantics;
use syntax::ast::{self, Pat};

use crate::IdeDatabase;

/// Enum types that implement `std::ops::Try` trait.
#[derive(Clone, Copy)]
pub enum TryEnum {
    Result,
    Option,
}

impl TryEnum {
    const ALL: [TryEnum; 2] = [TryEnum::Option, TryEnum::Result];

    /// Returns `Some(..)` if the provided type is an enum that implements `std::ops::Try`.
    pub fn from_ty(sema: &Semantics<IdeDatabase>, ty: &hir::Type) -> Option<TryEnum> {
        todo!()
    }

    pub fn happy_case(self) -> &'static str {
        match self {
            TryEnum::Result => "Ok",
            TryEnum::Option => "Some",
        }
    }

    pub fn sad_pattern(self) -> ast::Pat {
        todo!()
    }

    pub fn happy_pattern(self, pat: Pat) -> ast::Pat {
        todo!()
    }

    pub fn happy_pattern_wildcard(self) -> ast::Pat {
        todo!()
    }

    fn type_name(self) -> &'static str {
        match self {
            TryEnum::Result => "Result",
            TryEnum::Option => "Option",
        }
    }
}
