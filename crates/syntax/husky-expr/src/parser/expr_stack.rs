use husky_print_utils::p;
use salsa::DebugWithDb;

use super::*;

#[derive(Default, Debug)]
pub(crate) struct ExprStack {
    unfinished_exprs: Vec<(UnfinishedExpr, Precedence)>,
    finished_expr: Option<Expr>,
}

pub(super) enum TopExpr {
    Unfinished(UnfinishedExpr),
    Finished(Expr),
}

pub(super) enum TopExprRef<'a> {
    Unfinished(&'a UnfinishedExpr),
    Finished(&'a Expr),
    None,
}

impl From<Expr> for TopExpr {
    fn from(v: Expr) -> Self {
        Self::Finished(v)
    }
}

impl From<UnfinishedExpr> for TopExpr {
    fn from(v: UnfinishedExpr) -> Self {
        Self::Unfinished(v)
    }
}

impl ExprStack {
    pub(super) fn prev_unfinished_expr_precedence(&self) -> Option<Precedence> {
        self.unfinished_exprs
            .last()
            .map(|(_, precedence)| *precedence)
    }
}

impl Expr {
    pub fn base_entity_path(&self, db: &dyn WordDb, arena: &ExprArena) -> BaseEntityPath {
        match self {
            Expr::Literal(_) => BaseEntityPath::None,
            Expr::EntityPath {
                path: entity_path, ..
            } => match entity_path {
                Some(entity_path) => BaseEntityPath::Some(*entity_path),
                None => todo!(),
            },
            Expr::InheritedSymbol { .. } | Expr::CurrentSymbol { .. } => BaseEntityPath::None,
            Expr::SelfValue(_) => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::Binary {
                lopd,
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
                ropd,
            } => todo!(),
            Expr::Prefix {
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
                opd,
            } => todo!(),
            Expr::Suffix {
                opd,
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
            } => todo!(),
            Expr::Field { .. } => BaseEntityPath::None,
            Expr::MethodCall { .. } => BaseEntityPath::None,
            Expr::ExplicitApplication { function, argument } => todo!(),
            Expr::ExplicitApplicationOrRitchieCall { .. } => todo!(),
            // although unit is a valid entity,
            // but unit doesn't contains any subentity, so effectively none
            // ad hoc
            Expr::Unit { .. } => BaseEntityPath::None,
            Expr::NewTuple {
                lpar_token_idx,
                items,
                rpar_token_idx,
                ..
            } => todo!(),
            Expr::List { .. } => BaseEntityPath::None,
            Expr::Bracketed { item, .. } => arena[item].base_entity_path(db, arena),
            Expr::Err(e) => BaseEntityPath::Uncertain {
                inclination: match e {
                    ExprError::Original(OriginalExprError::UnrecognizedIdent { ident, .. }) => {
                        BaseEntityPathInclination::from_case(ident.case(db))
                    }
                    // ad hoc
                    _ => BaseEntityPathInclination::FunctionOrLocalValue,
                },
            },
            Expr::TemplateInstantiation { template, .. } => {
                arena[template].base_entity_path(db, arena)
            }
            Expr::Block { stmts } => BaseEntityPath::None,
            Expr::Be {
                src,
                be_token_idx,
                target,
            } => todo!(),
            Expr::BoxColonList { .. } => todo!(),
            Expr::FrameVarDecl {
                token_idx, ident, ..
            } => todo!(),
            Expr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                items: indices,
                rbox_token_idx,
            } => arena[owner].base_entity_path(db, arena),
            Expr::EmptyHtmlTag { .. } => BaseEntityPath::Err,
        }
    }
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(super) fn finished_expr(&self) -> Option<&Expr> {
        self.stack.finished_expr.as_ref()
    }

    pub(super) fn unfinished_exprs(&self) -> &[(UnfinishedExpr, Precedence)] {
        &self.stack.unfinished_exprs
    }

    pub(super) fn take_last_unfinished_expr(&mut self) -> Option<UnfinishedExpr> {
        self.stack.unfinished_exprs.pop().map(|(expr, _)| expr)
    }

    fn push_unfinished_expr(&mut self, unfinished_expr: UnfinishedExpr) {
        assert!(self.stack.finished_expr.is_none());
        let precedence = unfinished_expr.precedence();
        self.stack
            .unfinished_exprs
            .push((unfinished_expr, precedence))
    }

    pub(super) fn last_unfinished_expr(&self) -> Option<&UnfinishedExpr> {
        self.stack.unfinished_exprs.last().map(|(opr, _)| opr)
    }

    pub(super) fn last_unfinished_expr_mut(&mut self) -> Option<&mut UnfinishedExpr> {
        self.stack.unfinished_exprs.last_mut().map(|(opr, _)| opr)
    }

    pub(super) fn set_top_expr(&mut self, top_expr: TopExpr) {
        if let Some(function) = self.take_finished_expr() {
            self.push_unfinished_expr(UnfinishedExpr::Application { function });
        }
        match top_expr {
            TopExpr::Unfinished(unfinished_expr) => self.push_unfinished_expr(unfinished_expr),
            TopExpr::Finished(finished_expr) => self.stack.finished_expr = Some(finished_expr),
        }
    }

    pub(super) fn top_expr<'d>(&'d self) -> TopExprRef<'d> {
        if let Some(ref finished_expr) = self.stack.finished_expr {
            TopExprRef::Finished(finished_expr)
        } else if let Some((unfinished_expr, _)) = self.stack.unfinished_exprs.last() {
            TopExprRef::Unfinished(unfinished_expr)
        } else {
            TopExprRef::None
        }
    }

    pub(super) fn take_finished_expr(&mut self) -> Option<Expr> {
        std::mem::take(&mut self.stack.finished_expr)
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
            match self.stack.unfinished_exprs.pop().unwrap().0 {
                UnfinishedExpr::Binary {
                    lopd,
                    punctuation,
                    punctuation_token_idx,
                } => {
                    let lopd = self.alloc_expr(lopd);
                    let finished_expr = self.take_finished_expr();
                    self.stack.finished_expr = Some(match finished_expr {
                        Some(ropd) => Expr::Binary {
                            lopd,
                            opr: punctuation,
                            opr_token_idx: punctuation_token_idx,
                            ropd: self.alloc_expr(ropd),
                        },
                        None => Expr::Err(
                            OriginalExprError::NoRightOperandForBinaryOperator {
                                punctuation,
                                punctuation_token_idx,
                            }
                            .into(),
                        ),
                    })
                }
                UnfinishedExpr::Application { function } => {
                    let argument = self.take_finished_expr().unwrap();
                    let function = self.alloc_expr(function);
                    let argument = self.alloc_expr(argument);
                    self.stack.finished_expr =
                        Some(Expr::ExplicitApplication { function, argument })
                }
                UnfinishedExpr::Prefix {
                    punctuation,
                    punctuation_token_idx,
                } => {
                    let finished_expr = self.take_finished_expr();
                    self.stack.finished_expr = Some(match finished_expr {
                        Some(opd) => Expr::Prefix {
                            opr: punctuation,
                            opr_token_idx: punctuation_token_idx,
                            opd: self.alloc_expr(opd),
                        },
                        None => Expr::Err(
                            OriginalExprError::NoOperandForPrefixOperator {
                                prefix: punctuation,
                                prefix_token_idx: punctuation_token_idx,
                            }
                            .into(),
                        ),
                    })
                }
                UnfinishedExpr::ListItem {
                    separator_token_idx,
                } => todo!(),
                UnfinishedExpr::List { bra_token_idx, .. } => {
                    self.stack.finished_expr = Some(Expr::Err(
                        OriginalExprError::UnterminatedList { bra_token_idx }.into(),
                    ))
                }
                UnfinishedExpr::LambdaHead { inputs, start } => todo!(),
            }
        }
    }

    pub(super) fn replace_top_expr(&mut self, f: impl FnOnce(&mut Self, Option<Expr>) -> TopExpr) {
        let finished_expr = self.take_finished_expr();
        let top_expr = f(self, finished_expr);
        self.set_top_expr(top_expr)
    }

    pub(super) fn finish_batch(&mut self) -> Option<ExprIdx> {
        assert!(self.stack.unfinished_exprs.len() == 0);
        std::mem::take(&mut self.stack.finished_expr).map(|expr| self.parser.alloc_expr(expr))
    }

    pub(super) fn last_bra(&self) -> Option<Bracket> {
        for (unfinished_expr, _) in self.stack.unfinished_exprs.iter().rev() {
            match unfinished_expr {
                UnfinishedExpr::List {
                    opr,
                    bra,
                    bra_token_idx,
                    items,
                    commas,
                } => return Some(*bra),
                _ => (),
            }
        }
        None
    }

    pub(super) fn last_two_bras(&self) -> Vec<Bracket> {
        let mut bras = vec![];
        for (unfinished_expr, _) in self.stack.unfinished_exprs.iter().rev() {
            match unfinished_expr {
                UnfinishedExpr::List { bra, .. } => {
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
