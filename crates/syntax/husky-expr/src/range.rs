use crate::*;
use husky_token::{HasTokenIdxRange, RangedTokenSheet, TokenIdxRange, TokenSheetData};
use husky_vfs::ModulePath;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb, jar = ExprJar)]
pub struct ExprRangeRegion {
    expr_ranges: Vec<TokenIdxRange>,
}

#[salsa::tracked(jar = ExprJar, return_ref)]
pub(crate) fn expr_range_region(db: &dyn ExprDb, expr_region: ExprRegion) -> ExprRangeRegion {
    ExprRangeCalculator::new(db, expr_region).calc_all()
}

// #[test]
// fn expr_range_sheet_works() {
//     use tests::*;
//     DB::default().vfs_expect_test_debug_with_db("expr_range_sheet", todo!());
// }

impl std::ops::Index<ExprIdx> for ExprRangeRegion {
    type Output = TokenIdxRange;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_ranges[index.raw()]
    }
}

struct ExprRangeCalculator<'a> {
    token_sheet_data: &'a TokenSheetData,
    expr_region_data: &'a ExprRegionData,
    entity_path_expr_ranges: Vec<TokenIdxRange>,
    expr_ranges: Vec<TokenIdxRange>,
}

impl<'a> ExprRangeCalculator<'a> {
    fn new(db: &'a dyn ExprDb, expr_region: ExprRegion) -> Self {
        let expr_region_data = expr_region.data(db);
        let path = expr_region_data.path();
        let token_sheet_data = db.token_sheet_data(path.module_path(db)).unwrap();
        ExprRangeCalculator {
            token_sheet_data,
            expr_region_data,
            entity_path_expr_ranges: Default::default(),
            expr_ranges: Default::default(),
        }
    }

    fn calc_all(mut self) -> ExprRangeRegion {
        for entity_path_expr in self.expr_region_data.entity_path_expr_arena().iter() {
            todo!()
        }
        for expr in self.expr_region_data.expr_arena().iter() {
            self.expr_ranges.push(self.calc_expr_range(expr))
        }
        ExprRangeRegion {
            expr_ranges: self.expr_ranges,
        }
    }

    fn calc_expr_range(&self, expr: &Expr) -> TokenIdxRange {
        match expr {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => todo!(),
            Expr::InheritedSymbol {
                ident,
                token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => todo!(),
            Expr::CurrentSymbol {
                ident,
                token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            Expr::FrameVarDecl {
                token_idx,
                ident,
                frame_var_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => todo!(),
            Expr::BinaryOpn {
                lopd,
                opr,
                opr_token_idx,
                ropd,
            } => todo!(),
            Expr::Be {
                src,
                be_token_idx,
                target,
            } => todo!(),
            Expr::PrefixOpn {
                opr,
                opr_token_idx,
                opd,
            } => todo!(),
            Expr::SuffixOpn {
                opd,
                opr,
                punctuation_token_idx,
            } => todo!(),
            Expr::ApplicationOrRitchieCall {
                function,
                lpar_token_idx,
                argument,
                rpar_token_idx,
            } => todo!(),
            Expr::RitchieCall {
                function,
                implicit_arguments,
                lpar_token_idx,
                arguments,
                rpar_token_idx,
            } => todo!(),
            Expr::Field {
                owner,
                dot_token_idx,
                ident_token,
            } => todo!(),
            Expr::MethodCall {
                self_argument,
                dot_token_idx,
                ident_token,
                implicit_arguments,
                lpar_token_idx,
                nonself_arguments,
                rpar_token_idx,
            } => todo!(),
            Expr::TemplateInstantiation {
                template,
                implicit_arguments,
            } => todo!(),
            Expr::Application { function, argument } => todo!(),
            Expr::Bracketed {
                lpar_token_idx,
                item,
                rpar_token_idx,
            } => todo!(),
            Expr::NewTuple {
                lpar_token_idx,
                items,
                commas,
                rpar_token_idx,
            } => todo!(),
            Expr::NewBoxList {
                caller,
                lbox_token_idx,
                items,
                rbox_token_idx,
            } => todo!(),
            Expr::BoxColon {
                caller,
                lbox_token_idx,
                colon_token_idx,
                rbox_token,
            } => todo!(),
            Expr::Block { stmts } => todo!(),
            Expr::Err(_) => todo!(),
        }
    }
}
