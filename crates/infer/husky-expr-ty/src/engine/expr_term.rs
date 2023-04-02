mod explicit_application;
mod list;
mod prefix;

use husky_ty_expectation::TypePathDisambiguation;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr_term(
        &mut self,
        expr_idx: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) -> Option<LocalTerm> {
        #[cfg(test)]
        if self.expr_ty_infos.get(expr_idx).is_none() {
            print_debug_expr!(self, expr_idx);
            panic!("expect to infer type before infer term")
        }
        let term_result = self.calc_expr_term(expr_idx, local_term_region);
        let term = term_result.as_ref().ok().copied();
        self.save_new_expr_term(expr_idx, term_result);
        term
    }

    fn save_new_expr_term(&mut self, expr_idx: ExprIdx, term_result: ExprTermResult<LocalTerm>) {
        self.expr_terms.insert_new(expr_idx, term_result)
    }

    fn calc_expr_term(
        &mut self,
        expr_idx: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTermResult<LocalTerm> {
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
            } => Err(OriginalExprTermError::Todo.into()),
            Expr::CurrentSymbol {
                ident,
                token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => Err(OriginalExprTermError::Todo.into()),
            Expr::FrameVarDecl {
                token_idx,
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
            } => Err(OriginalExprTermError::Todo.into()),
            Expr::SelfType(_) => Err(OriginalExprTermError::Todo.into()),
            Expr::SelfValue(_) => Err(OriginalExprTermError::Todo.into()),
            Expr::Binary {
                lopd,
                opr,
                opr_token_idx,
                ropd,
            } => Err(OriginalExprTermError::Todo.into()),
            Expr::Be { .. } => todo!(),
            Expr::Prefix {
                opr,
                opr_token_idx,
                opd,
            } => self.calc_prefix_expr_term(expr_idx, opr, opd, local_term_region),
            Expr::Suffix {
                opd,
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
            } => Err(OriginalExprTermError::Todo.into()),
            Expr::ExplicitApplicationOrRitchieCall { .. } => {
                Err(OriginalExprTermError::Todo.into())
            }
            Expr::Field {
                owner,
                dot_token_idx,
                ident_token,
            } => Err(OriginalExprTermError::Todo.into()),
            Expr::MethodCall { .. } => Err(OriginalExprTermError::Todo.into()),
            Expr::TemplateInstantiation { .. } => Err(OriginalExprTermError::Todo.into()),
            Expr::ExplicitApplication { function, argument } => {
                // todo: implicit arguments
                self.calc_explicit_application_expr_term(
                    expr_idx,
                    function,
                    argument,
                    local_term_region,
                )
            }
            Expr::Bracketed {
                lpar_token_idx,
                item,
                rpar_token_idx,
            } => Err(OriginalExprTermError::Todo.into()),
            Expr::NewTuple { .. } => Err(OriginalExprTermError::Todo.into()),
            Expr::List { items, .. } => self.calc_list_expr_term(expr_idx, items),
            Expr::BoxColonList { .. } => Err(OriginalExprTermError::Todo.into()),
            Expr::Block { stmts } => Err(OriginalExprTermError::Todo.into()),
            Expr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                items: indices,
                rbox_token_idx,
            } => Err(OriginalExprTermError::Todo.into()),
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
    ) -> ExprTermResult<LocalTerm> {
        match path {
            Some(path) => match path {
                EntityPath::Module(_) => todo!(),
                EntityPath::ModuleItem(path) => match path {
                    ModuleItemPath::Type(path) => match self
                        .expr_disambiguation(expr_idx)
                        .map_err(|_| DerivedExprTermError::AmbiguousTypePath)?
                    {
                        ExprDisambiguation::TypePath(disambiguation) => Ok(match disambiguation {
                            TypePathDisambiguation::Ontology => TermEntityPath::TypeOntology(path),
                            TypePathDisambiguation::Constructor => {
                                TermEntityPath::TypeConstructor(path)
                            }
                        }
                        .into()),
                        _ => unreachable!(),
                    },
                    ModuleItemPath::Trait(_) => todo!(),
                    ModuleItemPath::Form(_) => todo!(),
                },
                EntityPath::AssociatedItem(_) => todo!(),
                EntityPath::TypeVariant(_) => todo!(),
            },
            //  Ok(Term::new(db,entity_path.into()).into()),
            None => todo!(),
        }
    }
}
