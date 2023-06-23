mod pattern_ty;

pub(crate) use self::pattern_ty::*;

use crate::*;
use husky_expr::*;
use husky_opn_syntax::{BinaryOpr, PrefixOpr};
use husky_print_utils::p;
use husky_token::{IntegerLikeLiteral, Literal};
use salsa::DebugWithDb;

pub(super) struct DeclarativeTermEngine<'a> {
    db: &'a dyn DeclarativeSignatureDb,
    expr_region_data: &'a ExprRegionData,
    declarative_term_menu: &'a DeclarativeTermMenu,
    symbol_declarative_term_region: SymbolDeclarativeTermRegion,
    expr_terms: ExprMap<DeclarativeTermResult2<DeclarativeTerm>>,
    /// todo: change this to ordered
    pattern_expr_ty_infos: PatternExprMap<PatternExprDeclarativeTypeInfo>,
    pattern_symbol_ty_infos: PatternSymbolMap<PatternSymbolTypeInfo>,
}

#[salsa::tracked(jar = DeclarativeSignatureJar, return_ref)]
pub(crate) fn declarative_term_region(
    db: &dyn DeclarativeSignatureDb,
    expr_region: ExprRegion,
) -> DeclarativeTermRegion {
    let expr_region_data = expr_region.data(db);
    let parent_expr_region = expr_region_data.parent();
    let parent_term_symbol_region =
        parent_expr_region.map(|r| declarative_term_region(db, r).term_symbol_region());
    let mut engine = DeclarativeTermEngine::new(db, expr_region, parent_term_symbol_region);
    engine.infer_all()
}

impl<'a> DeclarativeTermEngine<'a> {
    fn new(
        db: &'a dyn DeclarativeSignatureDb,
        expr_region: ExprRegion,
        parent_term_symbol_region: Option<&'a SymbolDeclarativeTermRegion>,
    ) -> Self {
        let toolchain = expr_region.toolchain(db);
        // ad hoc
        let _entity_path_menu = db.entity_path_menu(toolchain);
        let declarative_term_menu = db.declarative_term_menu(toolchain).unwrap();
        let expr_region_data = &expr_region.data(db);
        Self {
            db,
            expr_region_data,
            declarative_term_menu,
            symbol_declarative_term_region: SymbolDeclarativeTermRegion::new(
                parent_term_symbol_region,
                expr_region_data.symbol_region(),
            ),
            expr_terms: ExprMap::new(expr_region_data.expr_arena()),
            pattern_expr_ty_infos: PatternExprMap::new(expr_region_data.pattern_expr_arena()),
            pattern_symbol_ty_infos: PatternSymbolMap::new(
                expr_region_data
                    .pattern_expr_region()
                    .pattern_symbol_arena(),
            ),
        }
    }

    fn infer_all(mut self) -> DeclarativeTermRegion {
        self.init_current_symbol_terms();
        self.symbol_declarative_term_region.init_self_ty_and_value(
            self.db,
            self.expr_region_data.region_path(),
            self.expr_region_data.symbol_region(),
        );
        self.init_expr_roots();
        self.finish()
    }

    fn init_current_symbol_terms(&mut self) {
        let mut current_symbol_indexed_iter = self
            .expr_region_data
            .symbol_region()
            .current_symbol_indexed_iter();
        for (pattern_ty_constraint, symbols) in self
            .expr_region_data
            .symbol_region()
            .pattern_ty_constraints()
        {
            match pattern_ty_constraint {
                PatternTypeConstraint::ImplicitTypeParameter => {
                    let (current_symbol_idx, current_symbol) = current_symbol_indexed_iter
                        .next()
                        .expect("ty constraint should match with current symbols");
                    let CurrentSymbolVariant::ImplicitParameter {
                        implicit_parameter_variant,
                    } = current_symbol.variant() else {
                        unreachable!()
                    };
                    let ty = match implicit_parameter_variant {
                        CurrentImplicitParameterSymbol::Lifetime { label_token } => {
                            Ok(self.declarative_term_menu.lifetime_ty().into())
                        }
                        CurrentImplicitParameterSymbol::Type { ident_token } => {
                            Ok(self.declarative_term_menu.ty0().into())
                        }
                        CurrentImplicitParameterSymbol::Constant {
                            ident_token,
                            ty_expr_idx,
                        } => self.infer_new_expr_term(*ty_expr_idx).map_err(|_| todo!()),
                        _ => todo!(),
                    };
                    self.symbol_declarative_term_region
                        .add_new_implicit_parameter_symbol_signature(self.db, symbols.start(), ty)
                }
                PatternTypeConstraint::ExplicitParameter { pattern_expr, ty } => self
                    .init_current_symbol_signatures_in_explicit_parameter(
                        *pattern_expr,
                        *ty,
                        *symbols,
                    ),
                PatternTypeConstraint::LetVariables { .. }
                | PatternTypeConstraint::FrameVariable => {
                    // need only to compute for decl region
                    return;
                }
            }
        }
    }

    /// explicit parameters are infered in this crate;
    ///
    /// let variables, be variables and match variables are infered in `husky-expr-ty`
    fn init_current_symbol_signatures_in_explicit_parameter(
        &mut self,
        pattern_expr: PatternExprIdx,
        ty: ExprIdx,
        symbols: CurrentSymbolIdxRange,
    ) {
        let Ok(ty) = self.infer_new_expr_term(ty) else {
            for symbol in symbols {
                let modifier = self.expr_region_data[symbol].modifier();
                self.symbol_declarative_term_region.add_new_explicit_parameter_symbol_signature(
                    self.db,
                    symbol,
                    modifier,
                    Err(DeclarativeTermSymbolTypeErrorKind::SignatureDeclarativeTermError),
                )
            }
            return
        };
        self.infer_pattern_tys_in_explicit_parameter(pattern_expr, ty);
        for symbol in symbols {
            self.infer_current_symbol_signature_in_explicit_parameter(symbol)
        }
    }

    fn infer_current_symbol_signature_in_explicit_parameter(
        &mut self,
        current_symbol_idx: CurrentSymbolIdx,
    ) {
        let current_symbol = &self.expr_region_data.symbol_region()[current_symbol_idx];
        match current_symbol.variant() {
            CurrentSymbolVariant::ExplicitParameter {
                ident,
                pattern_symbol_idx,
            } => {
                let base_ty = self.pattern_symbol_ty_infos[pattern_symbol_idx].base_ty();
                self.symbol_declarative_term_region
                    .add_new_explicit_parameter_symbol_signature(
                        self.db,
                        current_symbol_idx,
                        current_symbol.modifier(),
                        Ok(base_ty),
                    )
            }
            _ => unreachable!("this function is only used for explicit parameters"),
        }
    }

    fn init_expr_roots(&mut self) {
        for expr_root in self.expr_region_data.roots() {
            match expr_root.kind() {
                ExprRootKind::RegularStructFieldType { .. }
                | ExprRootKind::SelfType
                | ExprRootKind::Trait
                | ExprRootKind::ReturnType
                | ExprRootKind::TupleStructFieldType
                | ExprRootKind::VarType => (),
                ExprRootKind::BlockExpr
                | ExprRootKind::LetStmtType
                | ExprRootKind::LetStmtInitialValue
                | ExprRootKind::HtmlArgumentExpr
                | ExprRootKind::ReturnExpr
                | ExprRootKind::Condition
                | ExprRootKind::ConstantImplicitParameterType
                | ExprRootKind::ExplicitParameterType
                | ExprRootKind::FieldBindInitialValue { .. }
                | ExprRootKind::Snippet
                | ExprRootKind::ValExpr
                | ExprRootKind::EvalExpr => continue,
                // ad hoc
                ExprRootKind::Traits => (),
            }
            self.cache_new_expr_term(expr_root.expr_idx())
        }
    }

    // infer the term for expr, assuming it hasn't been computed before
    fn infer_new_expr_term(
        &mut self,
        expr_idx: ExprIdx,
    ) -> DeclarativeTermResult2<DeclarativeTerm> {
        let result = self.calc_expr_term(expr_idx);
        let result_export = match result {
            Ok(term) => Ok(term),
            Err(_) => Err(DerivedDeclarativeTermError2::DeclarativeTermAbortion.into()),
        };
        self.save_expr_term(expr_idx, result);
        result_export
    }

    // cache the term for expr, assuming it hasn't been computed before
    fn cache_new_expr_term(&mut self, expr_idx: ExprIdx) {
        let result = self.calc_expr_term(expr_idx);
        self.save_expr_term(expr_idx, result)
    }

    pub(crate) fn finish(self) -> DeclarativeTermRegion {
        DeclarativeTermRegion::new(
            self.expr_region_data.region_path(),
            self.symbol_declarative_term_region,
            self.expr_terms,
            self.pattern_expr_ty_infos,
            self.pattern_symbol_ty_infos,
        )
    }

    fn save_expr_term(
        &mut self,
        expr_idx: ExprIdx,
        outcome: DeclarativeTermResult2<DeclarativeTerm>,
    ) {
        self.expr_terms.insert_new(expr_idx, outcome)
    }

    fn calc_expr_term(&mut self, expr_idx: ExprIdx) -> DeclarativeTermResult2<DeclarativeTerm> {
        match self.expr_region_data.expr_arena()[expr_idx] {
            Expr::Literal(_, literal) => match literal {
                Literal::Unit => todo!(),
                Literal::Char(_) => todo!(),
                Literal::String(_) => todo!(),
                Literal::Integer(literal) => match literal {
                    IntegerLikeLiteral::Unspecified => Ok(DeclarativeTerm::Literal(
                        UnresolvedTermLiteral::Integer.into(),
                    )),
                    IntegerLikeLiteral::I8(_) => todo!(),
                    IntegerLikeLiteral::I16(_) => todo!(),
                    IntegerLikeLiteral::I32(_) => todo!(),
                    IntegerLikeLiteral::I64(_) => todo!(),
                    IntegerLikeLiteral::I128(_) => todo!(),
                    IntegerLikeLiteral::ISize(_) => todo!(),
                    IntegerLikeLiteral::R8(_) => todo!(),
                    IntegerLikeLiteral::R16(_) => todo!(),
                    IntegerLikeLiteral::R32(_) => todo!(),
                    IntegerLikeLiteral::R64(_) => todo!(),
                    IntegerLikeLiteral::R128(_) => todo!(),
                    IntegerLikeLiteral::RSize(_) => todo!(),
                    IntegerLikeLiteral::U8(_) => todo!(),
                    IntegerLikeLiteral::U16(_) => todo!(),
                    IntegerLikeLiteral::U32(_) => todo!(),
                    IntegerLikeLiteral::U64(_) => todo!(),
                    IntegerLikeLiteral::U128(_) => todo!(),
                    IntegerLikeLiteral::USize(_) => todo!(),
                },
                Literal::Float(_) => todo!(),
                Literal::TupleIndex(_) => todo!(),
                Literal::Bool(_) => todo!(),
            },
            Expr::NonAssociatedEntity {
                entity_path_expr: _,
                path: entity_path,
            } => match entity_path {
                Some(entity_path) => Ok(DeclarativeTerm::EntityPath(match entity_path {
                    EntityPath::Module(_) => todo!(),
                    EntityPath::ModuleItem(path) => match path {
                        ModuleItemPath::Type(path) => {
                            /* ad hoc */
                            DeclarativeTermEntityPath::Type(path)
                        }
                        ModuleItemPath::Trait(path) => path.into(),
                        ModuleItemPath::Fugitive(path) => path.into(),
                    },
                    EntityPath::AssociatedItem(_) => todo!(),
                    EntityPath::TypeVariant(_) => todo!(),
                    EntityPath::ImplBlock(_) => todo!(),
                })),
                None => Err(DerivedDeclarativeTermError2::InvalidEntityPath.into()),
            },
            Expr::AssociatedItem {
                parent_expr_idx,
                scope_resolution_token,
                ident_token,
            } => todo!(),
            Expr::InheritedSymbol {
                inherited_symbol_idx,
                ..
            } => self
                .symbol_declarative_term_region
                .inherited_symbol_signature(inherited_symbol_idx)
                .term_symbol()
                .map(Into::into)
                .ok_or(DerivedDeclarativeTermError2::InheritedSymbolIsNotValidTerm.into()),
            Expr::CurrentSymbol {
                current_symbol_idx, ..
            } => Ok(self
                .symbol_declarative_term_region
                .current_symbol_signature(current_symbol_idx)
                .expect("not none")
                .term_symbol()
                .ok_or(OriginalDeclarativeTermError2::InvalidSymbolForTerm)?
                .into()),
            Expr::FrameVarDecl { .. } => unreachable!(),
            Expr::SelfType(_) => self
                .symbol_declarative_term_region
                .self_ty_term()
                .ok_or(DerivedDeclarativeTermError2::SelfTypeNotAllowedInThisRegion.into()),
            Expr::SelfValue(_) => self
                .symbol_declarative_term_region
                .self_ty_term()
                .ok_or(DerivedDeclarativeTermError2::SelfValueNotAllowedInThisRegion.into()),
            Expr::Binary {
                lopd, opr, ropd, ..
            } => {
                let Ok(lopd) = self.infer_new_expr_term(lopd) else {
                    return Err(DerivedDeclarativeTermError2::CannotInferOperandDeclarativeTermInPrefix.into());
                };
                let Ok(ropd) = self.infer_new_expr_term(ropd) else {
                    return Err(DerivedDeclarativeTermError2::CannotInferOperandDeclarativeTermInPrefix.into());
                };
                match opr {
                    BinaryOpr::Closed(_) => todo!(),
                    BinaryOpr::Shift(_) => todo!(),
                    BinaryOpr::Comparison(_) => todo!(),
                    BinaryOpr::ShortCircuitLogic(_) => todo!(),
                    BinaryOpr::Assign => todo!(),
                    BinaryOpr::AssignClosed(_) => todo!(),
                    BinaryOpr::AssignShift(_) => todo!(),
                    BinaryOpr::ScopeResolution => todo!(),
                    BinaryOpr::Curry => Ok(DeclarativeTermCurry::new_nondependent(
                        self.db,
                        CurryKind::Explicit, // ad hoc
                        Variance::Invariant, // ad hoc
                        lopd,
                        ropd,
                    )
                    .into()),
                    BinaryOpr::As => todo!(),
                    BinaryOpr::Ins => todo!(),
                    BinaryOpr::In => todo!(),
                }
            }
            Expr::Be { .. } => todo!(),
            Expr::Prefix {
                opr,
                opr_token_idx: _,
                opd,
            } => {
                let Ok(opd) = self.infer_new_expr_term(opd) else {
                    return Err(DerivedDeclarativeTermError2::CannotInferOperandDeclarativeTermInPrefix.into());
                };
                let tmpl = match opr {
                    PrefixOpr::Minus => todo!(),
                    PrefixOpr::Not => todo!(),
                    PrefixOpr::Tilde => DeclarativeTerm::LeashOrBitNot(
                        self.expr_region_data.region_path().toolchain(self.db),
                    ),
                    PrefixOpr::Ref => self.declarative_term_menu.ref_ty_path(),
                    PrefixOpr::Vector => todo!(),
                    PrefixOpr::Slice => todo!(),
                    PrefixOpr::CyclicSlice => todo!(),
                    PrefixOpr::Array(_) => todo!(),
                    PrefixOpr::Option => self.declarative_term_menu.option_ty_path(),
                };
                Ok(DeclarativeTermExplicitApplication::new(self.db, tmpl, opd).into())
            }
            Expr::Suffix {
                opd: _,
                opr: _punctuation,
                opr_token_idx: _punctuation_token_idx,
            } => todo!(),
            Expr::Field {
                owner: _self_expr,
                dot_token_idx: _,
                ident_token: _,
            } => todo!(),
            Expr::MethodApplicationOrCall { .. } => todo!(),
            Expr::TemplateInstantiation { .. } => todo!(),
            Expr::FunctionApplicationOrCall {
                function,
                ref implicit_arguments,
                items,
                ref commas,
                ..
            } => {
                let Ok(function) = self.infer_new_expr_term(function) else {
                    return Err(
                        DerivedDeclarativeTermError2::CannotInferArgumentDeclarativeTermInApplication.into()
                    )
                };
                let implicit_arguments = match implicit_arguments {
                    Some(implicit_arguments) => implicit_arguments
                        .arguments()
                        .into_iter()
                        .map(|_| todo!())
                        .collect(),
                    None => vec![],
                };
                assert!(items.len() <= commas.len() + 1);
                assert!(items.len() >= commas.len());
                let extra_comma = items.len() == commas.len();
                let items = items
                    .into_iter()
                    .map(|item| self.infer_new_expr_term(item))
                    .collect::<DeclarativeTermResult2<_>>()?;
                Ok(DeclarativeTermExplicitApplicationOrRitchieCall::new(
                    self.db,
                    function,
                    implicit_arguments,
                    items,
                    extra_comma,
                )
                .into())
            }
            Expr::ExplicitApplication { function, argument } => {
                let Ok(argument) = self.infer_new_expr_term(argument) else {
                    Err(DerivedDeclarativeTermError2::CannotInferArgumentDeclarativeTermInApplication)?
                };
                let Ok( function) = self.infer_new_expr_term(function) else {
                    Err(DerivedDeclarativeTermError2::CannotInferFunctionDeclarativeTermInApplication)?
                };
                Ok(DeclarativeTermExplicitApplication::new(self.db, function, argument).into())
            }
            Expr::NewTuple { items, .. } => {
                p!(self.expr_region_data.region_path().debug(self.db));
                p!(items.len());
                todo!()
            }
            Expr::List { items, .. } => {
                let items = items
                    .into_iter()
                    .map(|item| self.infer_new_expr_term(item))
                    .collect::<DeclarativeTermResult2<Vec<_>>>()?;
                Ok(DeclarativeTermList::new(
                    self.db,
                    self.expr_region_data.region_path().toolchain(self.db),
                    items,
                )
                .into())
            }
            Expr::BoxColonList { items, .. } => match items.len() {
                0 => Ok(self.declarative_term_menu.slice_ty_path()),
                _ => todo!(),
            },
            Expr::Bracketed { item, .. } => self.infer_new_expr_term(item),
            Expr::Block { stmts: _ } => todo!(),
            Expr::IndexOrCompositionWithList {
                owner: _,
                lbox_token_idx: _,
                items: _indices,
                rbox_token_idx: _,
            } => todo!(),
            Expr::Err(_) => Err(DerivedDeclarativeTermError2::ExprError.into()),
            Expr::Unit {
                lpar_token_idx,
                rpar_token_idx,
            } => todo!(),
            Expr::EmptyHtmlTag {
                empty_html_bra_idx: langle_token_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            Expr::FunctionCall { .. } => todo!(),
            Expr::Ritchie {
                ritchie_kind,
                parameter_ty_exprs,
                return_ty_expr,
                ..
            } => {
                let parameter_tys: SmallVec<[_; 2]> = parameter_ty_exprs
                    .into_iter()
                    .map(|argument_ty_expr| {
                        Ok(DeclarativeTermRitchieParameterContractedType::new(
                            // todo: handle &mut !!
                            Contract::Pure,
                            self.infer_new_expr_term(argument_ty_expr)?,
                        ))
                    })
                    .collect::<DeclarativeTermResult2<SmallVec<_>>>()?;
                let return_ty = match return_ty_expr {
                    Some(return_ty_expr) => self.infer_new_expr_term(return_ty_expr)?,
                    None => self.declarative_term_menu.unit(),
                };
                Ok(
                    DeclarativeTermRitchie::new(self.db, ritchie_kind, parameter_tys, return_ty)
                        .into(),
                )
            }
        }
    }

    pub(crate) fn current_symbol_term(&self, symbol: CurrentSymbolIdx) -> Option<SymbolSignature> {
        self.symbol_declarative_term_region
            .current_symbol_signature(symbol)
    }

    pub(crate) fn declarative_term_menu(&self) -> &DeclarativeTermMenu {
        self.declarative_term_menu
    }
}
