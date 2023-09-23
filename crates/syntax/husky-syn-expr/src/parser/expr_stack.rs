use crate::KeyedCallListItem;

use super::*;
use husky_print_utils::p;
use salsa::DebugWithDb;
use smallvec::smallvec;

/// stack based expression parsing.
///
/// finished expression if exists, sits on top of unfinished ones.
///
/// For example,
/// ```husky
/// a + b * (c + d
/// ```
/// in the above `c + d` would be the finished expression, `a +`, `b *` and `(c + d` would be unfinished expressions.
#[derive(Default, Debug)]
pub(crate) struct ExprStack {
    incomplete_exprs: Vec<(IncompleteSynExpr, Precedence)>,
    complete_expr: Option<SynExpr>,
}

pub(super) enum TopSynExpr {
    Unfinished(IncompleteSynExpr),
    Finished(SynExpr),
}

pub(super) enum TopExprRef<'a> {
    Incomplete(&'a IncompleteSynExpr),
    Finished(&'a SynExpr),
    None,
}

impl From<SynExpr> for TopSynExpr {
    fn from(v: SynExpr) -> Self {
        Self::Finished(v)
    }
}

impl From<IncompleteSynExpr> for TopSynExpr {
    fn from(v: IncompleteSynExpr) -> Self {
        Self::Unfinished(v)
    }
}

impl ExprStack {
    pub(super) fn prev_unfinished_expr_precedence(&self) -> Option<Precedence> {
        self.incomplete_exprs
            .last()
            .map(|(_, precedence)| *precedence)
    }
}

impl SynExpr {
    pub fn base_item_path(&self, db: &dyn SynExprDb, arena: &SynExprArena) -> BaseEntityPath {
        match self {
            SynExpr::Literal(_, _) => BaseEntityPath::None,
            SynExpr::PrincipalEntityPath { opt_path: path, .. } => match *path {
                Some(path) => BaseEntityPath::Some(path.into()),
                None => todo!(),
            },
            SynExpr::ScopeResolution {
                parent_expr_idx,
                colon_colon_regional_token,
                ident_token,
            } => todo!(),
            SynExpr::InheritedSymbol { .. } | SynExpr::CurrentSymbol { .. } => BaseEntityPath::None,
            SynExpr::SelfValue(_) => todo!(),
            SynExpr::SelfType(_) => BaseEntityPath::SelfType,
            SynExpr::Binary {
                lopd,
                opr,
                opr_regional_token_idx: punctuation_regional_token_idx,
                ropd,
            } => {
                match &arena[lopd] {
                    SynExpr::Literal(_, _) => todo!(),
                    SynExpr::PrincipalEntityPath {
                        item_path_expr,
                        opt_path,
                    } => todo!(),
                    SynExpr::ScopeResolution {
                        parent_expr_idx,
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
                    } => todo!(),
                    SynExpr::FrameVarDecl {
                        regional_token_idx,
                        ident,
                        frame_var_symbol_idx,
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
                    SynExpr::Be {
                        src,
                        be_regional_token_idx,
                        target,
                    } => todo!(),
                    SynExpr::Prefix {
                        opr,
                        opr_regional_token_idx,
                        opd,
                    } => todo!(),
                    SynExpr::Suffix {
                        opd,
                        opr,
                        opr_regional_token_idx,
                    } => todo!(),
                    SynExpr::FunctionApplicationOrCall {
                        function,
                        generic_arguments,
                        lpar_regional_token_idx,
                        items,
                        rpar_regional_token_idx,
                    } => todo!(),
                    SynExpr::Ritchie {
                        ritchie_kind_regional_token_idx,
                        ritchie_kind,
                        lpar_token,
                        parameter_ty_items,
                        rpar_regional_token_idx,
                        light_arrow_token,
                        return_ty_expr,
                    } => todo!(),
                    SynExpr::FunctionCall {
                        function,
                        generic_arguments,
                        lpar_regional_token_idx,
                        items,
                        rpar_regional_token_idx,
                    } => todo!(),
                    SynExpr::Field {
                        owner,
                        dot_regional_token_idx,
                        ident_token,
                    } => todo!(),
                    SynExpr::MethodApplicationOrCall {
                        self_argument,
                        dot_regional_token_idx,
                        ident_token,
                        generic_arguments,
                        lpar_regional_token_idx,
                        items,
                        rpar_regional_token_idx,
                    } => todo!(),
                    SynExpr::TemplateInstantiation {
                        template,
                        generic_arguments,
                    } => todo!(),
                    SynExpr::ExplicitApplication {
                        function_expr_idx,
                        argument_expr_idx,
                    } => todo!(),
                    SynExpr::At {
                        at_regional_token_idx,
                        place_label_regional_token,
                    } => todo!(),
                    SynExpr::Unit {
                        lpar_regional_token_idx,
                        rpar_regional_token_idx,
                    } => todo!(),
                    SynExpr::Bracketed {
                        lpar_regional_token_idx,
                        item,
                        rpar_regional_token_idx,
                    } => todo!(),
                    SynExpr::NewTuple {
                        lpar_regional_token_idx,
                        items,
                        rpar_regional_token_idx,
                    } => todo!(),
                    SynExpr::IndexOrCompositionWithList {
                        owner,
                        lbox_regional_token_idx,
                        items,
                        rbox_regional_token_idx,
                    } => todo!(),
                    SynExpr::List {
                        lbox_regional_token_idx,
                        items,
                        rbox_regional_token_idx,
                    } => todo!(),
                    SynExpr::BoxColonList {
                        lbox_regional_token_idx,
                        colon_regional_token_idx,
                        items,
                        rbox_regional_token_idx,
                    } => todo!(),
                    SynExpr::Block { stmts } => todo!(),
                    SynExpr::EmptyHtmlTag {
                        empty_html_bra_idx,
                        function_ident,
                        arguments,
                        empty_html_ket,
                    } => todo!(),
                    SynExpr::Sorry { regional_token_idx } => todo!(),
                    SynExpr::Todo { regional_token_idx } => todo!(),
                    SynExpr::Err(e) => {
                        p!(e.debug(db));
                        todo!()
                    }
                };
                todo!()
            }
            SynExpr::Prefix { .. } => todo!(),
            SynExpr::Suffix { .. } => todo!(),
            SynExpr::Field { .. } => BaseEntityPath::None,
            SynExpr::MethodApplicationOrCall { .. } => BaseEntityPath::None,
            SynExpr::ExplicitApplication {
                function_expr_idx: function,
                argument_expr_idx: argument,
            } => todo!(),
            SynExpr::FunctionApplicationOrCall { .. } => todo!(),
            SynExpr::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => todo!(),
            // although unit is a valid item,
            // but unit doesn't contains any subitem, so effectively none
            // ad hoc
            SynExpr::Unit { .. } => BaseEntityPath::None,
            SynExpr::NewTuple {
                lpar_regional_token_idx,
                items,
                rpar_regional_token_idx,
                ..
            } => todo!(),
            SynExpr::List { .. } => BaseEntityPath::None,
            SynExpr::Bracketed { item, .. } => arena[item].base_item_path(db, arena),
            SynExpr::Err(e) => BaseEntityPath::Uncertain {
                inclination: match e {
                    SynExprError::Original(OriginalSynExprError::UnrecognizedIdent {
                        ident,
                        ..
                    }) => BaseEntityPathInclination::from_case(ident.case(db)),
                    // ad hoc
                    _ => BaseEntityPathInclination::FunctionOrLocalValue,
                },
            },
            SynExpr::TemplateInstantiation { template, .. } => {
                arena[template].base_item_path(db, arena)
            }
            SynExpr::Block { stmts } => BaseEntityPath::None,
            SynExpr::Be {
                src,
                be_regional_token_idx,
                target,
            } => todo!(),
            SynExpr::BoxColonList { .. } => todo!(),
            SynExpr::FrameVarDecl {
                regional_token_idx,
                ident,
                ..
            } => todo!(),
            SynExpr::IndexOrCompositionWithList {
                owner,
                lbox_regional_token_idx,
                items: indices,
                rbox_regional_token_idx,
            } => arena[owner].base_item_path(db, arena),
            SynExpr::EmptyHtmlTag { .. } => BaseEntityPath::Err,
            SynExpr::FunctionCall { .. } => todo!(),
            SynExpr::Ritchie { .. } => todo!(),
            SynExpr::Sorry {
                regional_token_idx: regional_token_idx,
            } => todo!(),
            SynExpr::Todo {
                regional_token_idx: regional_token_idx,
            } => todo!(),
        }
    }
}

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(super) fn complete_expr(&self) -> Option<&SynExpr> {
        self.stack.complete_expr.as_ref()
    }

    pub(super) fn incomplete_exprs(&self) -> &[(IncompleteSynExpr, Precedence)] {
        &self.stack.incomplete_exprs
    }

    pub(super) fn take_last_incomplete_expr(&mut self) -> Option<IncompleteSynExpr> {
        self.stack.incomplete_exprs.pop().map(|(expr, _)| expr)
    }

    fn push_unfinished_expr(&mut self, incomplete_expr: IncompleteSynExpr) {
        assert!(self.stack.complete_expr.is_none());
        let precedence = incomplete_expr.precedence();
        self.stack
            .incomplete_exprs
            .push((incomplete_expr, precedence))
    }

    pub(super) fn last_incomplete_expr(&self) -> Option<&IncompleteSynExpr> {
        self.stack.incomplete_exprs.last().map(|(opr, _)| opr)
    }

    pub(super) fn last_incomplete_expr_mut(&mut self) -> Option<&mut IncompleteSynExpr> {
        self.stack.incomplete_exprs.last_mut().map(|(opr, _)| opr)
    }

    /// make `top_expr` the top expression.
    /// - if there is already a finished expression, interpret it as a function,
    /// and `top_expr` as an argument;
    /// - otherwise just adds it in the trivial way
    pub(super) fn push_top_syn_expr(&mut self, top_expr: TopSynExpr) {
        // this is for guaranteeing that application is left associative
        if self.complete_expr().is_some() {
            self.reduce(Precedence::Application)
        };
        if let Some(function) = self.take_complete_expr() {
            self.push_unfinished_expr(IncompleteSynExpr::Application { function });
        }
        match top_expr {
            TopSynExpr::Unfinished(unfinished_expr) => self.push_unfinished_expr(unfinished_expr),
            TopSynExpr::Finished(finished_expr) => self.stack.complete_expr = Some(finished_expr),
        }
    }

    /// if there's no need for the information of unfinished expressions, call `finished_expr` would be faster
    pub(super) fn top_expr<'d>(&'d self) -> TopExprRef<'d> {
        if let Some(ref finished_expr) = self.stack.complete_expr {
            TopExprRef::Finished(finished_expr)
        } else if let Some((unfinished_expr, _)) = self.stack.incomplete_exprs.last() {
            TopExprRef::Incomplete(unfinished_expr)
        } else {
            TopExprRef::None
        }
    }

    pub(super) fn take_complete_expr(&mut self) -> Option<SynExpr> {
        std::mem::take(&mut self.stack.complete_expr)
    }

    pub(super) fn set_complete_expr(&mut self, expr: SynExpr) {
        debug_assert!(self.complete_expr().is_none());
        self.stack.complete_expr = Some(expr)
    }

    fn reduce_aux(
        &mut self,
        f: impl Fn(&mut Self, Option<SynExpr>, IncompleteSynExpr) -> TopSynExpr,
    ) {
        let complete_expr = self.take_complete_expr();
        let Some((incomplete_expr, _)) = self.stack.incomplete_exprs.pop() else {
            unreachable!()
        };
        let top_expr = f(self, complete_expr, incomplete_expr);
        self.push_top_syn_expr(top_expr)
    }

    pub(super) fn reduce(&mut self, next_precedence: Precedence) {
        while let Some(prev_precedence) = self.stack.prev_unfinished_expr_precedence() {
            if prev_precedence < next_precedence {
                break;
            }
            // curry is right associative
            if prev_precedence == Precedence::Curry && next_precedence == Precedence::Curry {
                break;
            }
            match self.stack.incomplete_exprs.pop().unwrap().0 {
                IncompleteSynExpr::Binary {
                    lopd,
                    punctuation,
                    punctuation_regional_token_idx,
                } => {
                    let lopd = self.context_mut().alloc_expr(lopd);
                    let finished_expr = self.take_complete_expr();
                    self.stack.complete_expr = Some(match finished_expr {
                        Some(ropd) => SynExpr::Binary {
                            lopd,
                            opr: punctuation,
                            opr_regional_token_idx: punctuation_regional_token_idx,
                            ropd: self.context_mut().alloc_expr(ropd),
                        },
                        None => SynExpr::Err(
                            OriginalSynExprError::NoRightOperandForBinaryOperator {
                                punctuation,
                                punctuation_regional_token_idx,
                            }
                            .into(),
                        ),
                    })
                }
                IncompleteSynExpr::Application { function } => {
                    let argument = self.take_complete_expr().expect("");
                    let function = self.context_mut().alloc_expr(function);
                    let argument = self.context_mut().alloc_expr(argument);
                    self.stack.complete_expr = Some(SynExpr::ExplicitApplication {
                        function_expr_idx: function,
                        argument_expr_idx: argument,
                    })
                }
                IncompleteSynExpr::Prefix {
                    punctuation,
                    punctuation_regional_token_idx,
                } => {
                    let finished_expr = self.take_complete_expr();
                    self.stack.complete_expr = Some(match finished_expr {
                        Some(opd) => SynExpr::Prefix {
                            opr: punctuation,
                            opr_regional_token_idx: punctuation_regional_token_idx,
                            opd: self.context_mut().alloc_expr(opd),
                        },
                        None => SynExpr::Err(
                            OriginalSynExprError::NoOperandForPrefixOperator {
                                prefix: punctuation,
                                prefix_regional_token_idx: punctuation_regional_token_idx,
                            }
                            .into(),
                        ),
                    })
                }
                IncompleteSynExpr::CommaList {
                    bra_regional_token_idx,
                    ..
                } => {
                    self.stack.complete_expr = Some(SynExpr::Err(
                        OriginalSynExprError::UnterminatedList {
                            bra_regional_token_idx,
                        }
                        .into(),
                    ))
                }
                IncompleteSynExpr::LambdaHead { .. } => todo!(),
                IncompleteSynExpr::CallList {
                    lpar_regional_token_idx,
                    ..
                } => {
                    p!(prev_precedence, next_precedence);
                    todo!();
                    self.stack.complete_expr = Some(SynExpr::Err(
                        OriginalSynExprError::UnterminatedFunctionCallKeyedArgumentList {
                            bra_regional_token_idx: lpar_regional_token_idx,
                        }
                        .into(),
                    ))
                }
                IncompleteSynExpr::Ritchie {
                    ritchie_kind_regional_token_idx,
                    ritchie_kind,
                    lpar_token,
                    argument_tys,
                    rpar_regional_token_idx,
                    light_arrow_token,
                } => {
                    let finished_expr = self.take_complete_expr();
                    self.stack.complete_expr = Some(match finished_expr {
                        Some(return_ty) => SynExpr::Ritchie {
                            ritchie_kind_regional_token_idx,
                            ritchie_kind,
                            lpar_token,
                            parameter_ty_items: argument_tys,
                            rpar_regional_token_idx,
                            light_arrow_token: Some(light_arrow_token),
                            return_ty_expr: Some(self.context_mut().alloc_expr(return_ty)),
                        },
                        None => SynExpr::Err(
                            OriginalSynExprError::ExpectedTypeAfterLightArrow { light_arrow_token }
                                .into(),
                        ),
                    })
                }
                IncompleteSynExpr::KeyedArgument {
                    key_regional_token_idx,
                    key,
                    eq_token,
                } => {
                    self.reduce_aux(|this, opt_complete_expr, incomplete_expr| {
                        let Some(argument_expr) = opt_complete_expr else {
                            todo!()
                        };
                        let argument_expr_idx = this.context_mut().alloc_expr(argument_expr);
                        match incomplete_expr {
                            IncompleteSynExpr::CommaList {
                                opr: IncompleteCommaListOpr::FunctionApplicationOrCall { function },
                                bra,
                                bra_regional_token_idx,
                                mut items,
                            } => {
                                let mut items: SmallVec<[CallListItem; 4]> =
                                    items.into_iter().map(Into::into).collect();
                                items.push(
                                    KeyedCallListItem::new(
                                        key_regional_token_idx,
                                        key,
                                        argument_expr_idx,
                                        CallListSeparator::None,
                                    )
                                    .into(),
                                );
                                IncompleteSynExpr::CallList {
                                    opr: IncompleteCallListOpr::FunctionCall {
                                        function,
                                        generic_arguments: /* ad hoc */ None,
                                    },
                                    lpar_regional_token_idx: bra_regional_token_idx,
                                    items,
                                }
                                .into()
                            }
                            IncompleteSynExpr::CommaList {
                                opr:
                                    IncompleteCommaListOpr::MethodApplicationOrCall {
                                        self_expr,
                                        dot_regional_token_idx,
                                        ident_token,
                                        generic_arguments,
                                    },
                                bra,
                                bra_regional_token_idx,
                                items,
                            } => todo!(),
                            IncompleteSynExpr::CallList { .. } => todo!(),
                            _ => unreachable!(),
                        }
                    })
                }
            }
        }
    }

    /// use this when the incoming token might change the nature of the top expression
    pub(super) fn take_complete_and_push_to_top(
        &mut self,
        f: impl FnOnce(&mut Self, Option<SynExpr>) -> TopSynExpr,
    ) {
        let complete_expr = self.take_complete_expr();
        let top_expr = f(self, complete_expr);
        self.push_top_syn_expr(top_expr)
    }

    pub(super) fn finish_batch(&mut self) -> Option<SynExprIdx> {
        assert!(self.stack.incomplete_exprs.len() == 0);
        std::mem::take(&mut self.stack.complete_expr)
            .map(|expr| self.context_mut().alloc_expr(expr))
    }

    pub(super) fn last_bra(&self) -> Option<Bracket> {
        for (unfinished_expr, _) in self.stack.incomplete_exprs.iter().rev() {
            match unfinished_expr {
                IncompleteSynExpr::CommaList {
                    opr,
                    bra,
                    bra_regional_token_idx,
                    items,
                } => return Some(*bra),
                IncompleteSynExpr::CallList { .. } => return Some(Bracket::Par),
                _ => (),
            }
        }
        None
    }

    pub(super) fn last_two_bras(&self) -> Vec<Bracket> {
        let mut bras = vec![];
        for (unfinished_expr, _) in self.stack.incomplete_exprs.iter().rev() {
            match unfinished_expr {
                IncompleteSynExpr::CommaList { bra, .. } => {
                    bras.push(*bra);
                    if bras.len() >= 2 {
                        return bras;
                    }
                }
                _ => (),
            }
        }
        bras
    }
}
