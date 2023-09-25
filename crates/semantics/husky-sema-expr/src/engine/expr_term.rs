mod explicit_application;
mod list;
mod prefix;

use husky_token_data::{BoolLiteralData, FloatLiteralData};

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

    /// clear all holes before using this
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
            SynExpr::Literal(regional_token_idx, lit) => {
                Ok(
                    EtherealTerm::Literal(match lit {
                        LiteralData::Unit => TermLiteral::Unit,
                        LiteralData::Char(_) => todo!(),
                        LiteralData::String(val) => TermLiteral::String(val),
                        LiteralData::Integer(ilit) => match ilit {
                            IntegerLikeLiteralData::UnspecifiedRegular(val) => {
                                // todo: what if place is not none?
                                let ty = self
                                    .expr_ty_infos
                                    .get(expr_idx)
                                    .ok_or(DerivedExprTermError::TypeInfoNotInferred)?
                                    .ty()?;
                                let base_ty = ty.base_ty_data(self);
                                match base_ty {
                                    FluffyBaseTypeData::TypeOntology {
                                        ty_path,
                                        refined_ty_path:
                                            Left(PreludeTypePath::Num(PreludeNumTypePath::Int(
                                                int_ty_path,
                                            ))),
                                        ty_arguments,
                                        ty_ethereal_term,
                                    } => match int_ty_path {
                                        PreludeIntTypePath::I8 => todo!(),
                                        PreludeIntTypePath::I16 => todo!(),
                                        PreludeIntTypePath::I32 => {
                                            TermLiteral::I32(val.try_into().expect("todo"))
                                        }
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
                                    },
                                    _ => {
                                        #[cfg(test)]
                                        {
                                            p!(
                                                ty.debug(self.db),
                                                ty.show(self.db, self.fluffy_term_region.terms()),
                                                regional_token_idx,
                                                self.path()
                                            );
                                            todo!();
                                        }
                                        Err(DerivedExprTermError::LiteralTypeNotResolved)?
                                    }
                                }
                            }
                            IntegerLikeLiteralData::UnspecifiedLarge() => todo!(),
                            IntegerLikeLiteralData::I8(val) => TermLiteral::I8(val),
                            IntegerLikeLiteralData::I16(val) => TermLiteral::I16(val),
                            IntegerLikeLiteralData::I32(val) => TermLiteral::I32(val),
                            IntegerLikeLiteralData::I64(val) => {
                                TermLiteral::I64(TermI64Literal::new(self.db, val))
                            }
                            IntegerLikeLiteralData::I128(val) => {
                                TermLiteral::I128(TermI128Literal::new(self.db, val))
                            }
                            IntegerLikeLiteralData::ISize(val) => {
                                TermLiteral::ISize(TermISizeLiteral::new(self.db, val))
                            }
                            IntegerLikeLiteralData::R8(val) => TermLiteral::R8(val),
                            IntegerLikeLiteralData::R16(val) => TermLiteral::R16(val),
                            IntegerLikeLiteralData::R32(val) => TermLiteral::R32(val),
                            IntegerLikeLiteralData::R64(val) => TermLiteral::R64(todo!()),
                            IntegerLikeLiteralData::R128(val) => TermLiteral::R128(todo!()),
                            IntegerLikeLiteralData::RSize(val) => TermLiteral::RSize(todo!()),
                            IntegerLikeLiteralData::U8(val) => TermLiteral::U8(val),
                            IntegerLikeLiteralData::U16(val) => TermLiteral::U16(val),
                            IntegerLikeLiteralData::U32(val) => TermLiteral::U32(val),
                            IntegerLikeLiteralData::U64(val) => TermLiteral::U64(todo!()),
                            IntegerLikeLiteralData::U128(val) => TermLiteral::U128(todo!()),
                            IntegerLikeLiteralData::USize(val) => TermLiteral::USize(todo!()),
                        },
                        LiteralData::Float(lit) => {
                            match lit {
                                FloatLiteralData::Unspecified(lit) => {
                                    let ty = self
                                        .expr_ty_infos
                                        .get(expr_idx)
                                        .ok_or(DerivedExprTermError::TypeInfoNotInferred)?
                                        .ty()?;
                                    match ty.base_resolved(self) {
                                        FluffyTermBase::Ethereal(EtherealTerm::EntityPath(
                                            TermEntityPath::TypeOntology(ty_path),
                                        )) => {
                                            match ty_path.prelude_ty_path(self.db) {
                                                Some(prelude_ty_path) => {
                                                    match prelude_ty_path {
                                                        PreludeTypePath::Num(num_ty_path) => {
                                                            match num_ty_path {
                                                                PreludeNumTypePath::Int(_) => {
                                                                    todo!()
                                                                }
                                                                PreludeNumTypePath::Float(
                                                                    float_ty_path,
                                                                ) => {
                                                                    match float_ty_path {
                                                    PreludeFloatTypePath::F32 => TermLiteral::F32(
                                                        lit.data(self.db).parse().expect("todo"),
                                                    ),
                                                    PreludeFloatTypePath::F64 => TermLiteral::F64(
                                                        todo!(), // lit.data(self.db).parse().expect("todo"),
                                                    ),
                                                }
                                                                }
                                                            }
                                                        }
                                                        _ => todo!(),
                                                    }
                                                }
                                                None => todo!(),
                                            }
                                        }
                                        _ => Err(DerivedExprTermError::LiteralTypeNotResolved)?,
                                    }
                                }
                                FloatLiteralData::F32(val) => TermLiteral::F32(val),
                                FloatLiteralData::F64(_) => todo!(),
                            }
                        }
                        LiteralData::TupleIndex(_) => todo!(),
                        LiteralData::Bool(val) => match val {
                            BoolLiteralData::True => TermLiteral::Bool(true),
                            BoolLiteralData::False => TermLiteral::Bool(false),
                        },
                    })
                    .into(),
                )
            }
            SynExpr::PrincipalEntityPath {
                path_expr_idx,
                opt_path,
            } => self.calc_item_path_term(expr_idx, opt_path),
            SynExpr::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
            } => todo!(),
            SynExpr::InheritedSymbol {
                ident,
                regional_token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => todo!(),
            SynExpr::CurrentSymbol {
                ident,
                regional_token_idx,
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
                regional_token_idx,
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            SynExpr::SelfType(_) => todo!(),
            SynExpr::SelfValue(_) => todo!(),
            SynExpr::Binary {
                lopd,
                opr,
                opr_regional_token_idx,
                ropd,
            } => todo!(),
            SynExpr::Be { .. } => todo!(),
            SynExpr::Prefix {
                opr,
                opr_regional_token_idx,
                opd,
            } => self.calc_prefix_expr_term(expr_idx, opr, opd),
            SynExpr::Suffix {
                opd,
                opr: punctuation,
                opr_regional_token_idx: punctuation_regional_token_idx,
            } => todo!(),
            SynExpr::FunctionApplicationOrCall { .. } => todo!(),
            SynExpr::FunctionCall { .. } => todo!(),
            SynExpr::Field {
                owner,
                dot_regional_token_idx,
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
                lpar_regional_token_idx,
                item,
                rpar_regional_token_idx,
            } => Err(todo!()),
            SynExpr::NewTuple { .. } => todo!(),
            SynExpr::List { ref items, .. } => self.calc_list_expr_term(expr_idx, items),
            SynExpr::BoxColonList { .. } => todo!(),
            SynExpr::Block { stmts } => todo!(),
            SynExpr::IndexOrCompositionWithList {
                owner,
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => match self.expr_disambiguation(expr_idx) {
                Ok(SynExprDisambiguation::IndexOrComposeWithList(_)) => todo!(),
                Err(e) => Err(DerivedExprTermError::SemaExprError.into()),
                Ok(_) => unreachable!(),
            },
            SynExpr::EmptyHtmlTag {
                empty_html_bra_idx: langle_regional_token_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            SynExpr::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => todo!(),
            SynExpr::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            } => todo!(),
            SynExpr::Ritchie { .. } => todo!(),
            SynExpr::Sorry { regional_token_idx } => todo!(),
            SynExpr::Todo { regional_token_idx } => todo!(),
            SynExpr::Unreachable { regional_token_idx } => todo!(),
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
                        SynExprDisambiguation::TypePath(disambiguation) => {
                            Ok(match disambiguation {
                                TypePathDisambiguation::OntologyConstructor => {
                                    TermEntityPath::TypeOntology(path)
                                }
                                TypePathDisambiguation::InstanceConstructor => {
                                    TermEntityPath::TypeInstance(path)
                                }
                            }
                            .into())
                        }
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
