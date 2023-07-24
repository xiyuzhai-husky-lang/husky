mod explicit_application;
mod list;
mod prefix;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr_term(&mut self, expr_idx: SynExprIdx) -> Option<FluffyTerm> {
        #[cfg(test)]
        if self.expr_ty_infos.get(expr_idx).is_none() {
            print_debug_expr!(self, expr_idx);
            panic!("expect to infer type before infer term")
        }
        let term_result = self.calc_expr_term(expr_idx);
        let term = term_result.as_ref().ok().copied();
        self.save_new_expr_term(expr_idx, term_result);
        term
    }

    fn save_new_expr_term(
        &mut self,
        expr_idx: SynExprIdx,
        term_result: ExprTermResult<FluffyTerm>,
    ) {
        self.expr_terms.insert_new(expr_idx, term_result)
    }

    fn calc_expr_term(&mut self, expr_idx: SynExprIdx) -> ExprTermResult<FluffyTerm> {
        match self.expr_region_data[expr_idx] {
            SynExpr::Literal(_, _) => todo!(),
            SynExpr::PrincipalEntityPath {
                item_path_expr,
                opt_path,
            } => self.calc_item_path_term(expr_idx, opt_path),
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
            } => match self
                .declarative_term_region
                .current_symbol_signature(current_symbol_idx)
            {
                Some(current_symbol_signature) => match current_symbol_signature.term_symbol() {
                    Some(declarative_term_symbol) => Ok(EtherealTermSymbol::from_declarative(
                        self.db,
                        declarative_term_symbol,
                    )?
                    .into()),
                    None => todo!(),
                },
                None => todo!(),
            },
            SynExpr::FrameVarDecl {
                token_idx,
                ident,
                frame_var_symbol_idx: current_symbol_idx,
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
            SynExpr::Be { .. } => todo!(),
            SynExpr::Prefix {
                opr,
                opr_token_idx,
                opd,
            } => self.calc_prefix_expr_term(expr_idx, opr, opd),
            SynExpr::Suffix {
                opd,
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
            } => todo!(),
            SynExpr::FunctionApplicationOrCall { .. } => todo!(),
            SynExpr::FunctionCall { .. } => todo!(),
            SynExpr::Field {
                owner,
                dot_token_idx,
                ident_token,
            } => todo!(),
            SynExpr::MethodApplicationOrCall { .. } => todo!(),
            SynExpr::TemplateInstantiation { .. } => todo!(),
            SynExpr::ExplicitApplication {
                function_expr_idx: function,
                argument_expr_idx: argument,
            } => {
                // todo: implicit arguments
                self.calc_explicit_application_expr_term(expr_idx, function, argument)
            }
            SynExpr::Bracketed {
                lpar_token_idx,
                item,
                rpar_token_idx,
            } => Err(todo!()),
            SynExpr::NewTuple { .. } => todo!(),
            SynExpr::List { ref items, .. } => self.calc_list_expr_term(expr_idx, items),
            SynExpr::BoxColonList { .. } => todo!(),
            SynExpr::Block { stmts } => todo!(),
            SynExpr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                ref items,
                rbox_token_idx,
            } => match self.expr_disambiguation(expr_idx) {
                Ok(ExprDisambiguation::IndexOrComposeWithList(_)) => todo!(),
                Err(e) => Err(DerivedExprTermError::ExprTypeError.into()),
                Ok(_) => unreachable!(),
            },
            SynExpr::EmptyHtmlTag {
                empty_html_bra_idx: langle_token_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            SynExpr::Err(_) => Err(DerivedExprTermError::ExprError.into()),
            SynExpr::Unit {
                lpar_token_idx,
                rpar_token_idx,
            } => todo!(),
            SynExpr::Ritchie { .. } => todo!(),
        }
    }

    fn calc_item_path_term(
        &mut self,
        expr_idx: SynExprIdx,
        path: Option<PrincipalEntityPath>,
    ) -> ExprTermResult<FluffyTerm> {
        match path {
            Some(path) => match path {
                PrincipalEntityPath::Module(_) => todo!(),
                PrincipalEntityPath::ModuleItem(path) => match path {
                    ModuleItemPath::Type(path) => match self
                        .expr_ty_info_variant(expr_idx)
                        .map_err(|_| DerivedExprTermError::AmbiguousTypePath)?
                    {
                        ExprDisambiguation::TypePath(disambiguation) => Ok(match disambiguation {
                            TypePathDisambiguation::OntologyConstructor => {
                                TermEntityPath::TypeOntology(path)
                            }
                            TypePathDisambiguation::InstanceConstructor => {
                                TermEntityPath::TypeInstance(path)
                            }
                        }
                        .into()),
                        _ => unreachable!(),
                    },
                    ModuleItemPath::Trait(_) => todo!(),
                    ModuleItemPath::Fugitive(_) => todo!(),
                },
                PrincipalEntityPath::TypeVariant(_) => todo!(),
            },
            None => todo!(),
        }
    }
}
