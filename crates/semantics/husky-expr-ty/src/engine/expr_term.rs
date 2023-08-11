mod explicit_application;
mod list;
mod prefix;

use husky_token::{BoolLiteral, FloatLiteral};

use super::*;

impl<'a> ExprTypeEngine<'a> {
    /// perform this during finish stage
    pub(super) fn infer_expr_term(&mut self, syn_expr_idx: SynExprIdx) -> Option<FluffyTerm> {
        if let Some(term) = self.expr_terms.get(syn_expr_idx) {
            return term.as_ref().ok().copied();
        }
        let term_result = self.calc_expr_term(syn_expr_idx);
        let term = term_result.as_ref().ok().copied();
        self.save_new_expr_term(syn_expr_idx, term_result);
        term
    }

    pub(super) fn infer_extra_expr_terms_in_preparation_for_hir(&mut self) {
        for syn_expr_idx in self.expr_region_data.expr_arena().index_iter() {
            self.infer_extra_expr_term_in_preparation_for_hir(syn_expr_idx)
        }
    }

    // helpful for hir stage
    fn infer_extra_expr_term_in_preparation_for_hir(&mut self, syn_expr_idx: SynExprIdx) {
        if let Some(term) = self.expr_terms.get(syn_expr_idx) {
            return;
        }
        match self.expr_region_data[syn_expr_idx] {
            SynExpr::Literal(_, _) => (),
            _ => return,
        }
        let term_result = self.calc_expr_term(syn_expr_idx);
        let term = term_result.as_ref().ok().copied();
        self.save_new_expr_term(syn_expr_idx, term_result)
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
            SynExpr::Literal(token_idx, lit) => Ok(FluffyTerm::Literal(match lit {
                Literal::Unit => TermLiteral::Unit,
                Literal::Char(_) => todo!(),
                Literal::String(val) => TermLiteral::String(val),
                Literal::Integer(ilit) => match ilit {
                    IntegerLikeLiteral::UnspecifiedRegular(val) => {
                        let ty = self
                            .expr_ty_infos
                            .get(expr_idx)
                            .ok_or(DerivedExprTermError::TypeInfoNotInferred)?
                            .ty()?;
                        match ty {
                            FluffyTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
                                match ty_path.prelude_ty_path(self.db) {
                                    Some(prelude_ty_path) => match prelude_ty_path {
                                        PreludeTypePath::Num(num_ty_path) => match num_ty_path {
                                            PreludeNumTypePath::Int(int_ty_path) => {
                                                match int_ty_path {
                                                    PreludeIntTypePath::I8 => todo!(),
                                                    PreludeIntTypePath::I16 => todo!(),
                                                    PreludeIntTypePath::I32 => TermLiteral::I32(
                                                        val.try_into().expect("todo"),
                                                    ),
                                                    PreludeIntTypePath::I64 => todo!(),
                                                    PreludeIntTypePath::I128 => todo!(),
                                                    PreludeIntTypePath::ISize => {
                                                        TermLiteral::ISize(TermISizeLiteral::new(
                                                            self.db,
                                                            val.try_into().expect("ok"),
                                                        ))
                                                    }
                                                    PreludeIntTypePath::U8 => todo!(),
                                                    PreludeIntTypePath::U16 => todo!(),
                                                    PreludeIntTypePath::U32 => todo!(),
                                                    PreludeIntTypePath::U64 => todo!(),
                                                    PreludeIntTypePath::U128 => todo!(),
                                                    PreludeIntTypePath::USize => {
                                                        TermLiteral::USize(TermUSizeLiteral::new(
                                                            self.db,
                                                            val.try_into().expect("ok"),
                                                        ))
                                                    }
                                                    PreludeIntTypePath::R8 => todo!(),
                                                    PreludeIntTypePath::R16 => todo!(),
                                                    PreludeIntTypePath::R32 => todo!(),
                                                    PreludeIntTypePath::R64 => todo!(),
                                                    PreludeIntTypePath::R128 => todo!(),
                                                    PreludeIntTypePath::RSize => todo!(),
                                                }
                                            }
                                            PreludeNumTypePath::Float(_) => todo!(),
                                        },
                                        _ => todo!(),
                                    },
                                    None => todo!(),
                                }
                            }
                            _ => {
                                p!(
                                    ty.show(self.db, self.fluffy_term_region.terms()),
                                    token_idx,
                                    self.path()
                                );
                                todo!();
                                Err(DerivedExprTermError::LiteralTypeNotResolved)?
                            }
                        }
                    }
                    IntegerLikeLiteral::UnspecifiedLarge() => todo!(),
                    IntegerLikeLiteral::I8(val) => TermLiteral::I8(val),
                    IntegerLikeLiteral::I16(val) => TermLiteral::I16(val),
                    IntegerLikeLiteral::I32(val) => TermLiteral::I32(val),
                    IntegerLikeLiteral::I64(val) => {
                        TermLiteral::I64(TermI64Literal::new(self.db, val))
                    }
                    IntegerLikeLiteral::I128(val) => {
                        TermLiteral::I128(TermI128Literal::new(self.db, val))
                    }
                    IntegerLikeLiteral::ISize(val) => {
                        TermLiteral::ISize(TermISizeLiteral::new(self.db, val))
                    }
                    IntegerLikeLiteral::R8(val) => TermLiteral::R8(val),
                    IntegerLikeLiteral::R16(val) => TermLiteral::R16(val),
                    IntegerLikeLiteral::R32(val) => TermLiteral::R32(val),
                    IntegerLikeLiteral::R64(val) => TermLiteral::R64(todo!()),
                    IntegerLikeLiteral::R128(val) => TermLiteral::R128(todo!()),
                    IntegerLikeLiteral::RSize(val) => TermLiteral::RSize(todo!()),
                    IntegerLikeLiteral::U8(val) => TermLiteral::U8(val),
                    IntegerLikeLiteral::U16(val) => TermLiteral::U16(val),
                    IntegerLikeLiteral::U32(val) => TermLiteral::U32(val),
                    IntegerLikeLiteral::U64(val) => TermLiteral::U64(todo!()),
                    IntegerLikeLiteral::U128(val) => TermLiteral::U128(todo!()),
                    IntegerLikeLiteral::USize(val) => TermLiteral::USize(todo!()),
                },
                Literal::Float(lit) => match lit {
                    FloatLiteral::Unspecified(lit) => {
                        let ty = self
                            .expr_ty_infos
                            .get(expr_idx)
                            .ok_or(DerivedExprTermError::TypeInfoNotInferred)?
                            .ty()?;
                        match ty {
                            FluffyTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
                                match ty_path.prelude_ty_path(self.db) {
                                    Some(prelude_ty_path) => match prelude_ty_path {
                                        PreludeTypePath::Num(num_ty_path) => match num_ty_path {
                                            PreludeNumTypePath::Int(_) => todo!(),
                                            PreludeNumTypePath::Float(float_ty_path) => {
                                                match float_ty_path {
                                                    PreludeFloatTypePath::F32 => TermLiteral::F32(
                                                        lit.data(self.db).parse().expect("todo"),
                                                    ),
                                                    PreludeFloatTypePath::F64 => TermLiteral::F64(
                                                        todo!(), // lit.data(self.db).parse().expect("todo"),
                                                    ),
                                                }
                                            }
                                        },
                                        _ => todo!(),
                                    },
                                    None => todo!(),
                                }
                            }
                            _ => Err(DerivedExprTermError::LiteralTypeNotResolved)?,
                        }
                    }
                    FloatLiteral::F32(val) => TermLiteral::F32(val),
                    FloatLiteral::F64(_) => todo!(),
                },
                Literal::TupleIndex(_) => todo!(),
                Literal::Bool(val) => match val {
                    BoolLiteral::True => TermLiteral::Bool(true),
                    BoolLiteral::False => TermLiteral::Bool(false),
                },
            })),
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
            SynExpr::Unit {
                lpar_token_idx,
                rpar_token_idx,
            } => todo!(),
            SynExpr::Ritchie { .. } => todo!(),
            SynExpr::Sorry { token_idx } => todo!(),
            SynExpr::Todo { token_idx } => todo!(),
            SynExpr::Err(_) => Err(DerivedExprTermError::ExprError.into()),
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
                PrincipalEntityPath::MajorItem(path) => match path {
                    MajorItemPath::Type(path) => match self
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
                    MajorItemPath::Trait(_) => todo!(),
                    MajorItemPath::Fugitive(_) => todo!(),
                },
                PrincipalEntityPath::TypeVariant(_) => todo!(),
            },
            None => todo!(),
        }
    }
}
