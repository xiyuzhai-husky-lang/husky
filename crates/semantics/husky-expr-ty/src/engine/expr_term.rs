mod explicit_application;
mod list;
mod prefix;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr_term(&mut self, expr_idx: ExprIdx) -> Option<FluffyTerm> {
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

    fn save_new_expr_term(&mut self, expr_idx: ExprIdx, term_result: ExprTermResult<FluffyTerm>) {
        self.expr_terms.insert_new(expr_idx, term_result)
    }

    fn calc_expr_term(&mut self, expr_idx: ExprIdx) -> ExprTermResult<FluffyTerm> {
        match self.expr_region_data[expr_idx] {
            Expr::Literal(_, _) => todo!(),
            Expr::PrincipalEntityPath {
                entity_path_expr,
                opt_path,
            } => self.calc_entity_path_term(expr_idx, opt_path),
            Expr::ScopeResolution {
                parent_expr_idx,
                scope_resolution_token,
                ident_token,
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
            Expr::FrameVarDecl {
                token_idx,
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => todo!(),
            Expr::Binary {
                lopd,
                opr,
                opr_token_idx,
                ropd,
            } => todo!(),
            Expr::Be { .. } => todo!(),
            Expr::Prefix {
                opr,
                opr_token_idx,
                opd,
            } => self.calc_prefix_expr_term(expr_idx, opr, opd),
            Expr::Suffix {
                opd,
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
            } => todo!(),
            Expr::FunctionApplicationOrCall { .. } => todo!(),
            Expr::FunctionCall { .. } => todo!(),
            Expr::Field {
                owner,
                dot_token_idx,
                ident_token,
            } => todo!(),
            Expr::MethodApplicationOrCall { .. } => todo!(),
            Expr::TemplateInstantiation { .. } => todo!(),
            Expr::ExplicitApplication {
                function_expr_idx: function,
                argument_expr_idx: argument,
            } => {
                // todo: implicit arguments
                self.calc_explicit_application_expr_term(expr_idx, function, argument)
            }
            Expr::Bracketed {
                lpar_token_idx,
                item,
                rpar_token_idx,
            } => Err(todo!()),
            Expr::NewTuple { .. } => todo!(),
            Expr::List { ref items, .. } => self.calc_list_expr_term(expr_idx, items),
            Expr::BoxColonList { .. } => todo!(),
            Expr::Block { stmts } => todo!(),
            Expr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                ref items,
                rbox_token_idx,
            } => match self.expr_disambiguation(expr_idx) {
                Ok(ExprDisambiguation::IndexOrComposeWithList(_)) => todo!(),
                Err(e) => Err(DerivedExprTermError::ExprTypeError.into()),
                Ok(_) => unreachable!(),
            },
            Expr::EmptyHtmlTag {
                empty_html_bra_idx: langle_token_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            Expr::Err(_) => Err(DerivedExprTermError::ExprError.into()),
            Expr::Unit {
                lpar_token_idx,
                rpar_token_idx,
            } => todo!(),
            Expr::Ritchie { .. } => todo!(),
        }
    }

    fn calc_entity_path_term(
        &mut self,
        expr_idx: ExprIdx,
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
