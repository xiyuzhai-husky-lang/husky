mod accept;
mod debug;
mod disambiguate_token;
mod env;
mod expr_stack;
pub(crate) mod incomplete_expr;

use expr::VdSynExprIdx;
use latex_ast::ast::math::{LxMathAstIdx, LxMathAstIdxRange};
use range::VdSynExprAstRange;

use self::expr_stack::VdSynExprStack;
use crate::builder::VdSynExprBuilder;
use crate::*;

pub struct VdSynExprParser<'a, 'db> {
    builder: &'a mut VdSynExprBuilder<'db>,
    stack: VdSynExprStack,
}

impl<'a, 'db> VdSynExprParser<'a, 'db> {
    fn new(builder: &'a mut VdSynExprBuilder<'db>) -> Self {
        Self {
            builder,
            stack: Default::default(),
        }
    }
}

impl<'db> VdSynExprBuilder<'db> {
    pub fn parser(&mut self) -> VdSynExprParser<'_, 'db> {
        VdSynExprParser::new(self)
    }
}

impl<'a, 'db> VdSynExprParser<'a, 'db> {
    pub fn parse_asts(mut self, asts: LxMathAstIdxRange) -> VdSynExprIdx {
        for ast in asts {
            self.parse_ast(ast);
        }
        let Self { builder, stack } = self;
        let data = stack.finish();
        builder.alloc_expr(data, VdSynExprAstRange::Asts(asts.into()))
    }

    fn parse_ast(&mut self, ast: LxMathAstIdx) {
        let token = self.disambiguate_token(ast);
        self.stack.accept(token);
    }
}
