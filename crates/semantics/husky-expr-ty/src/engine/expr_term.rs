mod explicit_application;
mod list;
mod prefix;

use husky_ty_expectation::TypePathDisambiguation;

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
            Expr::Literal(_) => todo!(),
            Expr::EntityPath {
                entity_path_expr,
                path,
            } => self.calc_entity_path_term(expr_idx, path),
            Expr::InheritedSymbol {
                ident,
                token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => Err(todo!()),
            Expr::CurrentSymbol {
                ident,
                token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => Err(todo!()),
            Expr::FrameVarDecl {
                token_idx,
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
            } => Err(todo!()),
            Expr::SelfType(_) => Err(todo!()),
            Expr::SelfValue(_) => Err(todo!()),
            Expr::Binary {
                lopd,
                opr,
                opr_token_idx,
                ropd,
            } => Err(todo!()),
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
            } => Err(todo!()),
            Expr::ExplicitApplicationOrRitchieCall { .. } => Err(todo!()),
            Expr::Field {
                owner,
                dot_token_idx,
                ident_token,
            } => Err(todo!()),
            Expr::MethodCall { .. } => Err(todo!()),
            Expr::TemplateInstantiation { .. } => Err(todo!()),
            Expr::ExplicitApplication { function, argument } => {
                // todo: implicit arguments
                self.calc_explicit_application_expr_term(expr_idx, function, argument)
            }
            Expr::Bracketed {
                lpar_token_idx,
                item,
                rpar_token_idx,
            } => Err(todo!()),
            Expr::NewTuple { .. } => Err(todo!()),
            Expr::List { items, .. } => self.calc_list_expr_term(expr_idx, items),
            Expr::BoxColonList { .. } => Err(todo!()),
            Expr::Block { stmts } => Err(todo!()),
            Expr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                items: indices,
                rbox_token_idx,
            } => Err(todo!()),
            Expr::Err(_) => Err(DerivedExprTermError::ExprError.into()),
            Expr::Unit {
                lpar_token_idx,
                rpar_token_idx,
            } => todo!(),
        }
    }

    fn calc_entity_path_term(
        &mut self,
        expr_idx: ExprIdx,
        path: Option<EntityPath>,
    ) -> ExprTermResult<FluffyTerm> {
        match path {
            Some(path) => match path {
                EntityPath::Module(_) => todo!(),
                EntityPath::ModuleItem(path) => match path {
                    ModuleItemPath::Type(path) => match self
                        .expr_ty_info_variant(expr_idx)
                        .map_err(|_| DerivedExprTermError::AmbiguousTypePath)?
                    {
                        ExprTypeInfoVariant::TypePathDisambiguation(disambiguation) => {
                            Ok(match disambiguation {
                                TypePathDisambiguation::Ontology => {
                                    TermEntityPath::TypeOntology(path)
                                }
                                TypePathDisambiguation::Constructor => {
                                    TermEntityPath::TypeConstructor(path)
                                }
                            }
                            .into())
                        }
                        _ => unreachable!(),
                    },
                    ModuleItemPath::Trait(_) => todo!(),
                    ModuleItemPath::Form(_) => todo!(),
                },
                EntityPath::AssociatedItem(_) => todo!(),
                EntityPath::TypeVariant(_) => todo!(),
            },
            //  Ok(EtherealTerm::new(db,entity_path.into()).into()),
            None => todo!(),
        }
    }
}
