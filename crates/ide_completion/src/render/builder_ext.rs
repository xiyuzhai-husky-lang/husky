//! Extensions for `Builder` structure required for item rendering.

use either::Either;
use itertools::Itertools;

use crate::{context::CallKind, item::Builder, CompletionContext};

use syntax::ast;

#[derive(Debug)]
pub(super) enum Params {
    Named(Vec<(Either<ast::SelfParam, ast::Param>, hir::Param)>),
    Anonymous(usize),
}

impl Params {
    pub(super) fn len(&self) -> usize {
        match self {
            Params::Named(xs) => xs.len(),
            Params::Anonymous(len) => *len,
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Builder {
    fn should_add_parens(&self, ctx: &CompletionContext) -> bool {
        todo!()
    }

    pub(super) fn add_call_parens(
        &mut self,
        ctx: &CompletionContext,
        name: String,
        params: Params,
    ) -> &mut Builder {
        todo!()
    }
}
fn ref_of_param(ctx: &CompletionContext, arg: &str, ty: &hir::Type) -> &'static str {
    todo!()
}
