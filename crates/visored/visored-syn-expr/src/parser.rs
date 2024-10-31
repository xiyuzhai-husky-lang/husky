mod accept;
mod debug;
mod disambiguate_token;
mod env;
mod expr_stack;
pub(crate) mod incomplete_expr;

use self::expr_stack::VdSynExprStack;
use crate::builder::VdSynExprBuilder;
use crate::*;

pub struct VdSynExprParser<'a, 'db> {
    builder: &'a mut VdSynExprBuilder<'db>,
    stack: VdSynExprStack,
}
