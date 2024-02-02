mod explicit_application;
mod list;
mod prefix;

use husky_fly_term::{
    instantiation::FlyInstantiation, signature::binary_opr::SemaBinaryOprFlySignature,
};
use husky_term_prelude::literal::{
    float::TermF32Literal,
    int::{TermI128Literal, TermI64Literal, TermISizeLiteral, TermUSizeLiteral},
    TermLiteral,
};
use husky_token_data::{BoolLiteralTokenData, FloatLiteralTokenData};

use super::*;

impl<'a> SemaExprEngine<'a> {
    /// perform this during finish stage
    pub(crate) fn infer_expr_term(&mut self, sema_expr_idx: SemaExprIdx) -> Option<FlyTerm> {
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
        match sema_expr_idx.data_result(&self.sema_expr_arena) {
            Ok(SemaExprData::Literal(_, _)) => (),
            _ => return,
        }
        let term_result = self.calc_expr_term(sema_expr_idx);
        let term = term_result.as_ref().ok().copied();
        self.save_new_expr_term(sema_expr_idx, term_result)
    }

    fn save_new_expr_term(
        &mut self,
        expr_idx: SemaExprIdx,
        term_result: SemaExprTermResult<FlyTerm>,
    ) {
        self.sema_expr_term_results
            .insert_new((expr_idx, term_result))
            .expect("todo")
    }

    fn calc_expr_term(&mut self, sema_expr_idx: SemaExprIdx) -> SemaExprTermResult<FlyTerm> {
        let data = sema_expr_idx.data_result(&self.sema_expr_arena)?;
        match data {
            SemaExprData::Literal(regional_token_idx, lit) => {
                Ok(
                    EthTerm::Literal(match *lit {
                        LiteralTokenData::Unit => TermLiteral::Unit(()),
                        LiteralTokenData::Char(_) => todo!(),
                        LiteralTokenData::String(val) => TermLiteral::String(val),
                        LiteralTokenData::Integer(ilit) => match ilit {
                            IntegerLikeLiteralTokenData::UnspecifiedRegular(val) => {
                                // todo: what if place is not none?
                                let ty = sema_expr_idx
                                    .ok_ty(&self.sema_expr_arena)
                                    .ok_or(DerivedExprTermError::LiteralTypeNotInferred)?;
                                let base_ty = ty.base_ty_data(self);
                                match base_ty {
                                    FlyBaseTypeData::TypeOntology {
                                        ty_path,
                                        refined_ty_path:
                                            Left(PreludeTypePath::Num(PreludeNumTypePath::Int(
                                                int_ty_path,
                                            ))),
                                        ty_arguments,
                                        ty_ethereal_term,
                                    } => TermLiteral::from_unspecified_int(int_ty_path, val, self.db),
                                    _ => {
                                        Err(DerivedExprTermError::LiteralTypeNotResolved)?
                                    }
                                }
                            }
                            IntegerLikeLiteralTokenData::UnspecifiedLarge() => todo!(),
                            IntegerLikeLiteralTokenData::I8(val) => TermLiteral::I8(val),
                            IntegerLikeLiteralTokenData::I16(val) => TermLiteral::I16(val),
                            IntegerLikeLiteralTokenData::I32(val) => TermLiteral::I32(val),
                            IntegerLikeLiteralTokenData::I64(val) => {
                                TermLiteral::I64(TermI64Literal::new(self.db, val))
                            }
                            IntegerLikeLiteralTokenData::I128(val) => {
                                TermLiteral::I128(TermI128Literal::new(self.db, val))
                            }
                            IntegerLikeLiteralTokenData::ISize(val) => {
                                TermLiteral::ISize(TermISizeLiteral::new(self.db, val as i64))
                            }
                            IntegerLikeLiteralTokenData::R8(val) => TermLiteral::R8(val),
                            IntegerLikeLiteralTokenData::R16(val) => TermLiteral::R16(val),
                            IntegerLikeLiteralTokenData::R32(val) => TermLiteral::R32(val),
                            IntegerLikeLiteralTokenData::R64(val) => TermLiteral::R64(todo!()),
                            IntegerLikeLiteralTokenData::R128(val) => TermLiteral::R128(todo!()),
                            IntegerLikeLiteralTokenData::RSize(val) => TermLiteral::RSize(todo!()),
                            IntegerLikeLiteralTokenData::U8(val) => TermLiteral::U8(val),
                            IntegerLikeLiteralTokenData::U16(val) => TermLiteral::U16(val),
                            IntegerLikeLiteralTokenData::U32(val) => TermLiteral::U32(val),
                            IntegerLikeLiteralTokenData::U64(val) => TermLiteral::U64(todo!()),
                            IntegerLikeLiteralTokenData::U128(val) => TermLiteral::U128(todo!()),
                            IntegerLikeLiteralTokenData::USize(val) => TermLiteral::USize(todo!()),
                        },
                        LiteralTokenData::Float(lit) => {
                            match lit {
                                FloatLiteralTokenData::Unspecified(lit) => {
                                    let ty = sema_expr_idx
                                        .ok_ty(&self.sema_expr_arena)
                                        .ok_or(DerivedExprTermError::LiteralTypeNotInferred)?;
                                    match ty.base_resolved(self) {
                                        FlyTermBase::Ethereal(EthTerm::EntityPath(
                                            ItemPathTerm::TypeOntology(ty_path),
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
                                                        TermF32Literal::try_new(lit.text(self.db).to_string(),self.db) .expect("todo"),
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
                                FloatLiteralTokenData::F32(val) => TermLiteral::F32(val),
                                FloatLiteralTokenData::F64(val) => TermLiteral::F64(val),
                            }
                        }
                        LiteralTokenData::TupleIndex(_) => todo!(),
                        LiteralTokenData::Bool(val) => match val {
                            BoolLiteralTokenData::True => TermLiteral::Bool(true),
                            BoolLiteralTokenData::False => TermLiteral::Bool(false),
                        },
                    })
                    .into(),
                )
            }
            SemaExprData::PrincipalEntityPath {
                path_expr_idx,
                path,
                ty_path_disambiguation,
                ref instantiation,
            } => Ok(self.calc_item_path_term(
                *path,
                *ty_path_disambiguation,
                instantiation.as_ref(),
            )),
            SemaExprData::AssociatedItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
                static_dispatch,
            } => todo!(),
            &SemaExprData::InheritedSynSymbol {
                ident,
                regional_token_idx,
                inherited_syn_symbol_idx,
                inherited_syn_symbol_kind,
            } => Ok(self.symbol_terms[inherited_syn_symbol_idx]),
            SemaExprData::CurrentSynSymbol {
                ident,
                regional_token_idx,
                current_syn_symbol_idx,
                current_syn_symbol_kind,
            } => match self
                .declarative_term_region
                .current_syn_symbol_signature(*current_syn_symbol_idx)
            {
                Some(current_syn_symbol_signature) => {
                    match current_syn_symbol_signature.term_symbol() {
                        Some(declarative_term_symbol) => Ok(SymbolEthTerm::from_declarative(
                            self.db,
                            declarative_term_symbol,
                        )?
                        .into()),
                        None => todo!(),
                    }
                }
                None => todo!(),
            },
            SemaExprData::FrameVarDecl {
                regional_token_idx,
                ident,
                frame_var_symbol_idx: current_syn_symbol_idx,
                current_syn_symbol_kind,
            } => todo!(),
            SemaExprData::SelfType(regional_token_idx) => match self.self_ty {
                Some(self_ty_term) => Ok(self_ty_term.into()),
                None => {
                    p!(self
                        .declarative_term_region
                        .term_symbol_region()
                        .debug(self.db));
                    p!(self.path(), regional_token_idx);
                    todo!()
                }
            },
            SemaExprData::SelfValue(_) => todo!(),
            SemaExprData::Binary { dispatch, .. } => match dispatch.signature() {
                SemaBinaryOprFlySignature::Builtin => todo!(),
            },
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
            SemaExprData::Unveil { .. } => todo!(),
            SemaExprData::Unwrap { .. } => todo!(),
            SemaExprData::FunctionApplication {
                function_sema_expr_idx,
                argument_sema_expr_idx,
            } => {
                // todo: implicit arguments
                self.calc_explicit_application_expr_term(
                    *function_sema_expr_idx,
                    *argument_sema_expr_idx,
                )
            }
            SemaExprData::FunctionRitchieCall { .. } => todo!(),
            SemaExprData::Field { .. } => todo!(),
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
            SemaExprData::BoxColonList { items, .. } => match items.len() {
                0 => Ok(self.eth_term_menu().slice_ty_ontology().into()),
                1 => todo!(),
                2 => todo!(),
                _ => todo!(),
            },
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
            } => {
                let place = match place_label_regional_token {
                    Some(_) => todo!(),
                    None => match self.self_place {
                        Some(place) => place,
                        None => todo!(),
                    },
                };
                ApplicationEthTerm::new(
                    self.db,
                    self.eth_term_menu().at_ty_ontology(),
                    place.into(),
                )
                .map(Into::into)
                .map_err(Into::into)
            }
            SemaExprData::Unit { .. } => Ok(self.term_menu.unit_ty_ontology().into()),
            &SemaExprData::Ritchie {
                ritchie_kind,
                ref parameter_ty_items,
                return_ty_sema_expr_idx,
                ..
            } => {
                let mut params: Vec<FlyRitchieParameter> = vec![];
                for item in parameter_ty_items.clone() {
                    match self.infer_expr_term(item.sema_expr_idx) {
                        Some(ty_term) => params.push(
                            FlyRitchieRegularParameter::new(TermContract::Pure, ty_term).into(),
                        ),
                        None => todo!("err"),
                    }
                }
                let return_ty = match return_ty_sema_expr_idx {
                    Some(return_ty_sema_expr_idx) => {
                        match self.infer_expr_term(return_ty_sema_expr_idx) {
                            Some(return_ty) => return_ty,
                            None => todo!(),
                        }
                    }
                    None => self.eth_term_menu().unit_ty_ontology().into(),
                };
                FlyTerm::new_ritchie(self, ritchie_kind, params, return_ty).map_err(Into::into)
            }
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
            } => match items.len() {
                0 => unreachable!(),
                1 => {
                    let Some(size) = self.infer_expr_term(items[0].sema_expr_idx) else {
                        todo!()
                    };
                    FlyTerm::new_application(self, self.eth_term_menu().array_ty_ontology(), size)
                        .map_err(Into::into)
                }
                _ => todo!(),
            },
            SemaExprData::NewList {
                items, element_ty, ..
            } => todo!(),
        }
    }

    fn calc_item_path_term(
        &self,
        path: PrincipalEntityPath,
        ty_path_disambiguation: TypePathDisambiguation,
        instantiation: Option<&FlyInstantiation>,
    ) -> FlyTerm {
        let mut term = match path {
            PrincipalEntityPath::Module(_) => todo!(),
            PrincipalEntityPath::MajorItem(path) => match path {
                MajorItemPath::Type(path) => match ty_path_disambiguation {
                    TypePathDisambiguation::OntologyConstructor => ItemPathTerm::TypeOntology(path),
                    TypePathDisambiguation::InstanceConstructor => ItemPathTerm::TypeInstance(path),
                }
                .into(),
                MajorItemPath::Trait(trai_path) => ItemPathTerm::Trait(trai_path).into(),
                MajorItemPath::Fugitive(fugitive_path) => {
                    ItemPathTerm::Fugitive(fugitive_path).into()
                }
            },
            // todo: generics
            PrincipalEntityPath::TypeVariant(ty_variant_path) => {
                ItemPathTerm::TypeVariant(ty_variant_path).into()
            }
        };
        if let Some(instantiation) = instantiation
            && !instantiation.is_empty()
        {
            p!(path.debug(self.db));
            todo!()
        }
        term
    }
}
