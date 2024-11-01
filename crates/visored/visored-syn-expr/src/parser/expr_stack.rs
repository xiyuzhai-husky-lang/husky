use super::{
    error::{OriginalVdSynExprError, VdSynExprResult},
    expr::{VdSynExprData, VdSynExprIdx},
    incomplete_expr::IncompleteVdSynExprData,
    VdSynExprParser,
};
use visored_opr::{delimiter::VdLeftDelimiter, precedence::VdPrecedence};

#[derive(Default)]
pub(crate) struct VdSynExprStack {
    incomplete_exprs: Vec<(IncompleteVdSynExprData, VdPrecedence)>,
    complete_expr: Option<VdSynExprData>,
}

impl VdSynExprStack {
    pub fn finish(self) -> VdSynExprData {
        assert!(self.complete_expr.is_some());
        assert!(self.incomplete_exprs.is_empty());
        self.complete_expr.unwrap()
    }
}

pub(super) enum TopVdSynExpr {
    Unfinished(IncompleteVdSynExprData),
    Finished(VdSynExprData),
}

pub(super) enum TopExprRef<'a> {
    Incomplete(&'a IncompleteVdSynExprData),
    Finished(&'a VdSynExprData),
    None,
}

impl From<VdSynExprResult<VdSynExprData>> for TopVdSynExpr {
    fn from(result: VdSynExprResult<VdSynExprData>) -> Self {
        Self::Finished(match result {
            Ok(data) => data,
            Err(e) => VdSynExprData::Err(e),
        })
    }
}

impl From<VdSynExprData> for TopVdSynExpr {
    fn from(v: VdSynExprData) -> Self {
        Self::Finished(v)
    }
}

impl From<IncompleteVdSynExprData> for TopVdSynExpr {
    fn from(v: IncompleteVdSynExprData) -> Self {
        Self::Unfinished(v)
    }
}

impl VdSynExprStack {
    pub(super) fn prev_unfinished_expr_precedence(&self) -> Option<VdPrecedence> {
        self.incomplete_exprs
            .last()
            .map(|(_, precedence)| *precedence)
    }
}

impl<'a, 'db> VdSynExprParser<'a, 'db> {
    pub(super) fn complete_expr(&self) -> Option<&VdSynExprData> {
        self.stack.complete_expr.as_ref()
    }

    pub(super) fn incomplete_exprs(&self) -> &[(IncompleteVdSynExprData, VdPrecedence)] {
        &self.stack.incomplete_exprs
    }

    pub(super) fn take_last_incomplete_expr(&mut self) -> Option<IncompleteVdSynExprData> {
        self.stack.incomplete_exprs.pop().map(|(expr, _)| expr)
    }

    fn push_unfinished_expr(&mut self, incomplete_expr: IncompleteVdSynExprData) {
        assert!(self.stack.complete_expr.is_none());
        let precedence = incomplete_expr.precedence();
        self.stack
            .incomplete_exprs
            .push((incomplete_expr, precedence))
    }

    pub(super) fn last_incomplete_expr(&self) -> Option<&IncompleteVdSynExprData> {
        self.stack.incomplete_exprs.last().map(|(opr, _)| opr)
    }

    pub(super) fn last_incomplete_expr_mut(&mut self) -> Option<&mut IncompleteVdSynExprData> {
        self.stack.incomplete_exprs.last_mut().map(|(opr, _)| opr)
    }

    /// make `top_expr` the top expression.
    /// - if there is already a finished expression, interpret it as a function,
    /// and `top_expr` as an argument;
    /// - otherwise just adds it in the trivial way
    pub(super) fn push_top_syn_expr(&mut self, top_expr: TopVdSynExpr) {
        // this is for guaranteeing that application is left associative
        if self.complete_expr().is_some() {
            self.reduce(VdPrecedence::APPLICATION)
        };
        if let Some(function) = self.take_complete_expr() {
            todo!()
            // self.push_unfinished_expr(IncompleteVdSynExprData::Application { function });
        }
        match top_expr {
            TopVdSynExpr::Unfinished(unfinished_expr) => self.push_unfinished_expr(unfinished_expr),
            TopVdSynExpr::Finished(finished_expr) => self.stack.complete_expr = Some(finished_expr),
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

    pub(super) fn take_complete_expr(&mut self) -> Option<VdSynExprData> {
        std::mem::take(&mut self.stack.complete_expr)
    }

    pub(super) fn set_complete_expr(&mut self, expr: VdSynExprData) {
        debug_assert!(self.complete_expr().is_none());
        self.stack.complete_expr = Some(expr)
    }

    fn reduce_aux(
        &mut self,
        f: impl Fn(&mut Self, Option<VdSynExprData>, IncompleteVdSynExprData) -> TopVdSynExpr,
    ) {
        let complete_expr = self.take_complete_expr();
        let Some((incomplete_expr, _)) = self.stack.incomplete_exprs.pop() else {
            unreachable!()
        };
        let top_expr = f(self, complete_expr, incomplete_expr);
        self.push_top_syn_expr(top_expr)
    }

    pub(super) fn reduce(&mut self, next_precedence: VdPrecedence) {
        while let Some(prev_precedence) = self.stack.prev_unfinished_expr_precedence() {
            if prev_precedence < next_precedence {
                break;
            }
            // // curry is right associative
            // if prev_precedence == VdPrecedence::Curry && next_precedence == VdPrecedence::Curry {
            //     break;
            // }
            match self.stack.incomplete_exprs.pop().unwrap().0 {
                IncompleteVdSynExprData::Binary { lopd, opr } => {
                    let finished_expr = self.take_complete_expr();
                    self.stack.complete_expr = Some(match finished_expr {
                        Some(ropd) => VdSynExprData::Binary {
                            lopd,
                            opr,
                            ropd: self.builder.alloc_expr(ropd, todo!()),
                        },
                        None => VdSynExprData::Err(
                            OriginalVdSynExprError::NoRightOperandForBinaryOperator { opr }.into(),
                        ),
                    })
                }
                IncompleteVdSynExprData::Prefix { opr } => {
                    let finished_expr = self.take_complete_expr();
                    self.stack.complete_expr = Some(match finished_expr {
                        Some(opd) => VdSynExprData::Prefix {
                            opr,
                            opd: self.builder.alloc_expr(opd, todo!()),
                        },
                        None => VdSynExprData::Err(
                            OriginalVdSynExprError::NoOperandForPrefixOperator { opr }.into(),
                        ),
                    })
                }
                IncompleteVdSynExprData::SeparatedList { bra_token_idx, .. } => {
                    self.stack.complete_expr = Some(VdSynExprData::Err(
                        OriginalVdSynExprError::UnterminatedList { bra_token_idx }.into(),
                    ))
                }
                IncompleteVdSynExprData::CallList { .. } => todo!(),
            }
        }
    }

    /// use this when the incoming token might change the nature of the top expression
    pub(super) fn take_complete_and_push_to_top(
        &mut self,
        f: impl FnOnce(&mut Self, Option<VdSynExprData>) -> TopVdSynExpr,
    ) {
        let complete_expr = self.take_complete_expr();
        let top_expr = f(self, complete_expr);
        self.push_top_syn_expr(top_expr)
    }

    pub(super) fn finish_batch(&mut self) -> Option<VdSynExprIdx> {
        assert!(self.stack.incomplete_exprs.len() == 0);
        std::mem::take(&mut self.stack.complete_expr)
            .map(|expr| self.builder.alloc_expr(expr, todo!()))
    }

    pub(super) fn last_bra(&self) -> Option<VdLeftDelimiter> {
        for (unfinished_expr, _) in self.stack.incomplete_exprs.iter().rev() {
            match unfinished_expr {
                IncompleteVdSynExprData::SeparatedList { bra, .. } => return Some(*bra),
                IncompleteVdSynExprData::CallList { .. } => todo!(),
                //  return Some(Delimiter::Par),
                _ => (),
            }
        }
        None
    }

    pub(super) fn last_two_bras(&self) -> Vec<VdLeftDelimiter> {
        let mut bras = vec![];
        for (unfinished_expr, _) in self.stack.incomplete_exprs.iter().rev() {
            match unfinished_expr {
                IncompleteVdSynExprData::SeparatedList { bra, .. } => {
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
