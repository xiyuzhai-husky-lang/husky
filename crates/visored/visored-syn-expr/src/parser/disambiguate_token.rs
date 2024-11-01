use latex_ast::ast::math::LxMathAstIdx;
use visored_annotation::annotation::space::VdSpaceAnnotation;
use visored_opr::{
    delimiter::{VdLeftDelimiter, VdRightDelimiter},
    opr::VdOpr,
    separator::VdSeparator,
};

use super::{
    expr::{VdSynExprClass, VdSynExprData, VdSynExprIdx},
    VdSynExprParser,
};

pub struct DisambiguatedMathAst {
    ast: LxMathAstIdx,
    preceding_space_annotation: Option<VdSpaceAnnotation>,
}

pub enum DisambiguatedToken {
    Expr(VdSynExprData, VdSynExprClass),
    Opr(VdOpr),
    Separator(VdSeparator),
    LeftDelimiter(VdLeftDelimiter),
    RightDelimiter(VdRightDelimiter),
}

impl<'a, 'db> VdSynExprParser<'a, 'db> {
    pub fn disambiguate_token(&mut self, ast: LxMathAstIdx) -> DisambiguatedToken {
        todo!()
    }
}
