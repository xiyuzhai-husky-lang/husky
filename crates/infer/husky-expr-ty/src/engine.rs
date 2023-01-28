use crate::*;

pub(crate) struct ExprTypeEngine<'a> {
    db: &'a dyn ExprTypeDb,
    expr_region_data: &'a ExprRegionData,
    expr_ty_infos: ExprMap<ExprTypeInfo>,
    unresolved_term_table: UnresolvedTermTable,
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn new(db: &'a dyn ExprTypeDb, expr_region: ExprRegion) -> Self {
        let expr_region_data = expr_region.data(db);
        Self {
            db,
            expr_region_data,
            expr_ty_infos: ExprMap::new(expr_region_data.expr_arena()),
            unresolved_term_table: Default::default(),
        }
    }

    pub(crate) fn infer_all(&mut self) {
        for root in self.expr_region_data.roots() {
            let ty = self.infer_new(root.expr(), None);
            // todo: check coherence
        }
    }

    fn infer_new(
        &mut self,
        expr_idx: ExprIdx,
        expectation: Option<Expectation>,
    ) -> ExprTypeResult<LocalTerm> {
        let ty_result = self.calc(expr_idx, expectation.as_ref());
        let ty_result_out = match ty_result {
            Ok(ty) => Ok(ty),
            Err(_) => Err(DerivedExprTypeError::TypeInfoErr.into()),
        };
        let opt_expectation = self.unresolved_term_table.intern_expection(expectation);
        self.save(expr_idx, ExprTypeInfo::new(ty_result, opt_expectation));
        ty_result_out
    }

    fn save(&mut self, expr_idx: ExprIdx, info: ExprTypeInfo) {
        self.expr_ty_infos.insert_new(expr_idx, info)
    }

    pub(crate) fn finish(self) -> ExprTypeRegion {
        ExprTypeRegion::new(self.expr_region_data.path(), self.expr_ty_infos)
    }

    fn calc(
        &mut self,
        expr_idx: ExprIdx,
        expection: Option<&Expectation>,
    ) -> ExprTypeResult<LocalTerm> {
        let expr = &self.expr_region_data[expr_idx];
        match expr {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => match entity_path {
                Some(entity_path) => match self.db.entity_ty(*entity_path) {
                    Ok(ty) => Ok(ty.into()),
                    Err(_) => todo!(),
                },
                None => todo!(),
            },
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
                current_symbol_idx,
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
                punctuation,
                punctuation_token_idx,
            } => todo!(),
            Expr::ApplicationOrFunctionCall {
                function,
                lpar_token_idx,
                argument,
                rpar_token_idx,
            } => todo!(),
            Expr::FunctionCall {
                function,
                implicit_arguments,
                lpar_token_idx,
                arguments,
                rpar_token_idx,
            } => todo!(),
            Expr::Field {
                this_expr,
                dot_token_idx,
                ident_token,
            } => todo!(),
            Expr::MethodCall {
                this_expr,
                dot_token_idx,
                ident_token,
                implicit_arguments,
                lpar_token_idx,
                arguments,
                rpar_token_idx,
            } => todo!(),
            Expr::TemplateInstantiation {
                template,
                implicit_arguments,
            } => todo!(),
            Expr::Application { function, argument } => {
                let x = todo!();
                todo!();
            }
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
            Expr::Err(_) => Err(DerivedExprTypeError::ExprError.into()),
        }
    }
}
