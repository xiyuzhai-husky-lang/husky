mod explicit_application;
mod list;
mod prefix;

use husky_token_data::{BoolLiteralData, FloatLiteralData};

use super::*;

impl<'a> SemaExprEngine<'a> {
    /// perform this during finish stage
    pub(super) fn infer_expr_term(&mut self, sema_expr_idx: SemaExprIdx) -> Option<FluffyTerm> {
        if let Some(term_result) = self.sema_expr_term_results.get_value(sema_expr_idx) {
            return term_result.as_ref().ok().copied();
        }
        let term_result = self.calc_expr_term(sema_expr_idx);
        let term = term_result.as_ref().ok().copied();
        self.save_new_expr_term(sema_expr_idx, term_result);
        term
    }

    /// clear all holes before using this
    pub(super) fn infer_extra_expr_terms_in_preparation_for_hir(&mut self) {
        for sema_expr_idx in self.sema_expr_arena.index_iter() {
            self.infer_extra_expr_term_in_preparation_for_hir(sema_expr_idx)
        }
    }

    // helpful for hir stage
    fn infer_extra_expr_term_in_preparation_for_hir(&mut self, sema_expr_idx: SemaExprIdx) {
        if let Some(_) = self.sema_expr_term_results.get_value(sema_expr_idx) {
            return;
        }
        // ad hoc
        match sema_expr_idx.data(self.sema_expr_arena.arena_ref()) {
            SemaExprData::Literal(_, _) => (),
            _ => return,
        }
        let term_result = self.calc_expr_term(sema_expr_idx);
        let term = term_result.as_ref().ok().copied();
        self.save_new_expr_term(sema_expr_idx, term_result)
    }

    fn save_new_expr_term(
        &mut self,
        expr_idx: SemaExprIdx,
        term_result: SemaExprTermResult<FluffyTerm>,
    ) {
        self.sema_expr_term_results
            .insert_new((expr_idx, term_result))
            .expect("todo")
    }

    fn calc_expr_term(&mut self, sema_expr_idx: SemaExprIdx) -> SemaExprTermResult<FluffyTerm> {
        let Ok(data) = sema_expr_idx.data_result(&self.sema_expr_arena) else {
            return todo!();
        };
        match data {
            SemaExprData::Literal(regional_token_idx, lit) => {
                Ok(
                    EtherealTerm::Literal(match *lit {
                        LiteralData::Unit => TermLiteral::Unit,
                        LiteralData::Char(_) => todo!(),
                        LiteralData::String(val) => TermLiteral::String(val),
                        LiteralData::Integer(ilit) => match ilit {
                            IntegerLikeLiteralData::UnspecifiedRegular(val) => {
                                // todo: what if place is not none?
                                let ty = sema_expr_idx
                                    .ok_ty(&self.sema_expr_arena)
                                    .ok_or(DerivedExprTermError::LiteralTypeNotInferred)?;
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
                                    let ty = sema_expr_idx
                                        .ok_ty(&self.sema_expr_arena)
                                        .ok_or(DerivedExprTermError::LiteralTypeNotInferred)?;
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
            SemaExprData::PrincipalEntityPath {
                path_expr_idx,
                path,
                ty_path_disambiguation,
            } => Ok(self.calc_item_path_term(*path, *ty_path_disambiguation)),
            SemaExprData::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
                static_dispatch,
            } => todo!(),
            SemaExprData::InheritedSymbol {
                ident,
                regional_token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => todo!(),
            SemaExprData::CurrentSymbol {
                ident,
                regional_token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => match self
                .declarative_term_region
                .current_symbol_signature(*current_symbol_idx)
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
            SemaExprData::FrameVarDecl {
                regional_token_idx,
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            SemaExprData::SelfType(_) => todo!(),
            SemaExprData::SelfValue(_) => todo!(),
            SemaExprData::Binary {
                lopd,
                opr,
                opr_regional_token_idx,
                ropd,
            } => todo!(),
            SemaExprData::Be { .. } => todo!(),
            SemaExprData::Prefix {
                opr,
                opr_regional_token_idx,
                opd_sema_expr_idx,
            } => self.calc_prefix_expr_term(sema_expr_idx, *opr, *opd_sema_expr_idx),
            SemaExprData::Suffix {
                opd_sema_expr_idx,
                opr,
                opr_regional_token_idx,
            } => todo!(),
            SemaExprData::Application {
                function_sema_expr_idx,
                argument_sema_expr_idx,
            } => {
                // todo: implicit arguments
                self.calc_explicit_application_expr_term(
                    *function_sema_expr_idx,
                    *argument_sema_expr_idx,
                )
            }
            SemaExprData::FnCall { .. } => todo!(),
            SemaExprData::GnCall { .. } => todo!(),
            SemaExprData::Field {
                owner_sema_expr_idx,
                dot_regional_token_idx,
                ident_token,
                field_dispatch,
            } => todo!(),
            SemaExprData::MethodApplication { .. } => todo!(),
            SemaExprData::MethodFnCall { .. } => todo!(),
            SemaExprData::MethodGnCall { .. } => todo!(),
            SemaExprData::TemplateInstantiation { .. } => todo!(),
            SemaExprData::Bracketed {
                lpar_regional_token_idx,
                item,
                rpar_regional_token_idx,
            } => Err(todo!()),
            SemaExprData::NewTuple { .. } => todo!(),
            SemaExprData::NewList { ref items, .. } => {
                self.calc_new_list_expr_term(sema_expr_idx, items)
            }
            SemaExprData::BoxColonList { .. } => todo!(),
            SemaExprData::Block { stmts } => todo!(),
            SemaExprData::Index {
                owner_sema_expr_idx,
                lbox_regional_token_idx,
                index_sema_list_items: indices,
                rbox_regional_token_idx,
                index_dynamic_dispatch,
            } => todo!(),
            SemaExprData::CompositionWithList {
                owner,
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
            // match self.expr_disambiguation(expr_idx) {
            //     Ok(SemaExprData::IndexOrComposeWithList(_)) => todo!(),
            //     Err(e) => Err(DerivedExprTermError::SemaExprError.into()),
            //     Ok(_) => unreachable!(),
            // },
            SemaExprData::EmptyHtmlTag {
                empty_html_bra_idx: langle_regional_token_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            SemaExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => todo!(),
            SemaExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            } => todo!(),
            SemaExprData::Ritchie { .. } => todo!(),
            SemaExprData::Sorry { regional_token_idx } => todo!(),
            SemaExprData::Todo { regional_token_idx } => todo!(),
            SemaExprData::Unreachable { regional_token_idx } => todo!(),
            SemaExprData::VecFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
            } => Ok(self.term_menu.list_ty_ontology().into()),
            SemaExprData::ArrayFunctor {
                lbox_regional_token_idx,
                items,
                rbox_regional_token_idx,
            } => todo!(),
            SemaExprData::NewList {
                lbox_regional_token_idx,
                items,
                rbox_regional_token_idx,
            } => todo!(),
        }
    }

    fn calc_item_path_term(
        &mut self,
        path: PrincipalEntityPath,
        ty_path_disambiguation: TypePathDisambiguation,
    ) -> FluffyTerm {
        match path {
            PrincipalEntityPath::Module(_) => todo!(),
            PrincipalEntityPath::MajorItem(path) => match path {
                MajorItemPath::Type(path) => match ty_path_disambiguation {
                    TypePathDisambiguation::OntologyConstructor => {
                        TermEntityPath::TypeOntology(path)
                    }
                    TypePathDisambiguation::InstanceConstructor => {
                        TermEntityPath::TypeInstance(path)
                    }
                }
                .into(),
                MajorItemPath::Trait(_) => todo!(),
                MajorItemPath::Fugitive(_) => todo!(),
            },
            PrincipalEntityPath::TypeVariant(_) => todo!(),
        }
    }
}
