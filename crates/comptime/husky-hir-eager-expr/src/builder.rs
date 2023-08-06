use crate::*;
use husky_expr_ty::ExprTypeRegion;
use husky_syn_expr::{SynExpr, SynExprIdx, SynExprRegion, SynExprRegionData, SynStmt, SynStmtIdx};

pub struct HirEagerExprBuilder<'a> {
    db: &'a dyn HirEagerExprDb,
    syn_expr_region_data: &'a SynExprRegionData,
    expr_ty_region: &'a ExprTypeRegion,
    expr_arena: HirEagerExprArena,
    stmt_arena: HirEagerStmtArena,
}

impl<'a> HirEagerExprBuilder<'a> {
    pub fn new(db: &'a dyn HirEagerExprDb, syn_expr_region: SynExprRegion) -> Self {
        Self {
            db,
            syn_expr_region_data: syn_expr_region.data(db),
            expr_ty_region: db.expr_ty_region(syn_expr_region),
            expr_arena: Default::default(),
            stmt_arena: Default::default(),
        }
    }

    pub fn finish(self) -> HirEagerExprRegion {
        HirEagerExprRegion::new(self.db)
    }

    pub fn new_expr(&mut self, syn_expr_idx: SynExprIdx) -> HirEagerExprIdx {
        let expr = match self.syn_expr_region_data[syn_expr_idx] {
            SynExpr::Literal(_, _) => todo!(),
            SynExpr::PrincipalEntityPath {
                item_path_expr,
                opt_path,
            } => todo!(),
            SynExpr::ScopeResolution {
                parent_expr_idx,
                scope_resolution_token,
                ident_token,
            } => todo!(),
            SynExpr::InheritedSymbol {
                ident,
                token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => todo!(),
            SynExpr::CurrentSymbol {
                ident,
                token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            SynExpr::FrameVarDecl {
                token_idx,
                ident,
                frame_var_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            SynExpr::SelfType(_) => todo!(),
            SynExpr::SelfValue(_) => todo!(),
            SynExpr::Binary {
                lopd,
                opr,
                opr_token_idx,
                ropd,
            } => todo!(),
            SynExpr::Be {
                src,
                be_token_idx,
                ref target,
            } => todo!(),
            SynExpr::Prefix {
                opr,
                opr_token_idx,
                opd,
            } => todo!(),
            SynExpr::Suffix {
                opd,
                opr,
                opr_token_idx,
            } => todo!(),
            SynExpr::FunctionApplicationOrCall {
                function,
                ref generic_arguments,
                lpar_token_idx,
                ref items,
                rpar_token_idx,
            } => todo!(),
            SynExpr::Ritchie {
                ritchie_kind_token_idx,
                ritchie_kind,
                lpar_token,
                ref parameter_ty_items,
                rpar_token_idx,
                light_arrow_token,
                return_ty_expr,
            } => todo!(),
            SynExpr::FunctionCall {
                function,
                ref generic_arguments,
                lpar_token_idx,
                ref items,
                rpar_token_idx,
            } => todo!(),
            SynExpr::Field {
                owner,
                dot_token_idx,
                ident_token,
            } => todo!(),
            SynExpr::MethodApplicationOrCall {
                self_argument,
                dot_token_idx,
                ident_token,
                ref generic_arguments,
                lpar_token_idx,
                ref items,
                rpar_token_idx,
            } => todo!(),
            SynExpr::TemplateInstantiation {
                template,
                ref generic_arguments,
            } => todo!(),
            SynExpr::ExplicitApplication {
                function_expr_idx,
                argument_expr_idx,
            } => todo!(),
            SynExpr::Unit {
                lpar_token_idx,
                rpar_token_idx,
            } => todo!(),
            SynExpr::Bracketed {
                lpar_token_idx,
                item,
                rpar_token_idx,
            } => todo!(),
            SynExpr::NewTuple {
                lpar_token_idx,
                ref items,
                rpar_token_idx,
            } => todo!(),
            SynExpr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                ref items,
                rbox_token_idx,
            } => todo!(),
            SynExpr::List {
                lbox_token_idx,
                ref items,
                rbox_token_idx,
            } => todo!(),
            SynExpr::BoxColonList {
                lbox_token_idx,
                colon_token_idx,
                ref items,
                rbox_token_idx,
            } => todo!(),
            SynExpr::Block { stmts } => {
                let mut syn_stmt_indices: Vec<SynStmtIdx> = vec![];
                let mut hir_eager_stmts: Vec<HirEagerStmt> = vec![];
                for syn_stmt_idx in stmts {
                    match self.new_stmt(syn_stmt_idx) {
                        Some(hir_eager_stmt) => {
                            syn_stmt_indices.push(syn_stmt_idx);
                            hir_eager_stmts.push(hir_eager_stmt)
                        }
                        None => todo!(),
                    }
                }
                // todo: record syn_stmt_indices in source map
                HirEagerExpr::Block {
                    stmts: self.stmt_arena.alloc_batch(hir_eager_stmts),
                }
            }
            SynExpr::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            SynExpr::Err(_) => todo!(),
        };
        self.expr_arena.alloc_one(expr)
    }

    fn new_stmt(&mut self, syn_stmt_idx: SynStmtIdx) -> Option<HirEagerStmt> {
        Some(match self.syn_expr_region_data[syn_stmt_idx] {
            SynStmt::Let {
                let_token,
                ref let_variable_pattern,
                ref assign_token,
                initial_value,
            } => todo!(),
            SynStmt::Return {
                return_token,
                result,
            } => HirEagerStmt::Return {
                result: self.new_expr(result),
            },
            SynStmt::Require {
                require_token,
                condition,
            } => HirEagerStmt::Require {
                condition: self.new_expr(condition),
            },
            SynStmt::Assert {
                assert_token,
                condition,
            } => HirEagerStmt::Assert {
                condition: self.new_expr(condition),
            },
            SynStmt::Break { break_token } => HirEagerStmt::Break,
            SynStmt::Eval {
                expr_idx,
                eol_semicolon,
            } => HirEagerStmt::Eval {
                expr_idx: self.new_expr(expr_idx),
            },
            SynStmt::ForBetween {
                for_token,
                ref particulars,
                frame_var_symbol_idx,
                ref eol_colon,
                ref block,
            } => todo!(),
            SynStmt::ForIn {
                for_token,
                ref condition,
                ref eol_colon,
                ref block,
            } => todo!(),
            SynStmt::ForExt {
                forext_token,
                expr,
                ref eol_colon,
                ref block,
            } => todo!(),
            SynStmt::While {
                while_token,
                ref condition,
                ref eol_colon,
                ref block,
            } => todo!(),
            SynStmt::DoWhile {
                do_token,
                while_token,
                ref condition,
                ref eol_colon,
                ref block,
            } => todo!(),
            SynStmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => todo!(),
            SynStmt::Match { match_token } => todo!(),
            SynStmt::Err(_) => todo!(),
        })
    }
}
