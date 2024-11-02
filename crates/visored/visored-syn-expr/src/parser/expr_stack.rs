use super::{
    error::{OriginalVdSynExprError, VdSynExprResult},
    expr::{VdSynExprArenaRef, VdSynExprData, VdSynExprIdx},
    incomplete_expr::IncompleteVdSynExprData,
    VdSynExprParser,
};
use crate::expr::VdSynExprClass;
use either::*;
use smallvec::smallvec;
use visored_annotation::annotation::space::VdSpaceAnnotation;
use visored_opr::{
    delimiter::VdBaseLeftDelimiter,
    precedence::{VdPrecedence, VdPrecedenceRange},
    separator::{VdBaseSeparator, VdSeparator},
};

#[derive(Default, Debug)]
pub(crate) struct VdSynExprStack {
    incomplete_exprs: Vec<(IncompleteVdSynExprData, VdPrecedence)>,
    complete_expr: Option<VdSynExprData>,
}

impl VdSynExprStack {
    pub fn finish(self) -> VdSynExprData {
        assert!(self.complete_expr.is_some());
        assert!(
            self.incomplete_exprs.is_empty(),
            "incomplete exprs should be empty but {:?}, complete expr: {:?}",
            self.incomplete_exprs,
            self.complete_expr
        );
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
    pub(super) fn push_top_syn_expr(
        &mut self,
        preceding_space_annotation: Option<VdSpaceAnnotation>,
        top_expr: TopVdSynExpr,
    ) {
        // this is for guaranteeing that application is left associative
        if self.complete_expr().is_some() {
            match preceding_space_annotation {
                Some(annotation) => todo!(),
                _ => {
                    self.reduce(VdPrecedenceRange::SPACE_LEFT, Some(VdSeparator::SPACE));
                }
            }
        };
        if let Some(expr) = self.take_complete_expr() {
            match preceding_space_annotation {
                Some(annotation) => todo!(),
                _ => {
                    let expr = self.builder.alloc_expr(expr);
                    self.push_unfinished_expr(IncompleteVdSynExprData::SeparatedList {
                        separator: VdBaseSeparator::Space.into(),
                        fragments: smallvec![Left(expr)],
                    })
                }
            }
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

    // fn reduce_aux(
    //     &mut self,
    //     f: impl Fn(&mut Self, Option<VdSynExprData>, IncompleteVdSynExprData) -> TopVdSynExpr,
    // ) {
    //     let complete_expr = self.take_complete_expr();
    //     let Some((incomplete_expr, _)) = self.stack.incomplete_exprs.pop() else {
    //         unreachable!()
    //     };
    //     let top_expr = f(self, complete_expr, incomplete_expr);
    //     self.push_top_syn_expr(top_expr)
    // }

    pub(super) fn reduce(
        &mut self,
        precedence_range: VdPrecedenceRange,
        separator1: Option<VdSeparator>,
    ) {
        while let Some(prev_precedence) = self.stack.prev_unfinished_expr_precedence() {
            if !precedence_range.include(prev_precedence) {
                break;
            }
            let (incomplete_expr, precedence) = self.stack.incomplete_exprs.pop().unwrap();
            match incomplete_expr {
                IncompleteVdSynExprData::Binary { lopd, opr } => {
                    let complete_expr = self.take_complete_expr();
                    self.stack.complete_expr = Some(match complete_expr {
                        Some(ropd) => VdSynExprData::Binary {
                            lopd,
                            opr,
                            ropd: self.builder.alloc_expr(ropd),
                        },
                        None => VdSynExprData::Err(
                            OriginalVdSynExprError::NoRightOperandForBinaryOperator { opr }.into(),
                        ),
                    })
                }
                IncompleteVdSynExprData::Prefix { opr } => {
                    let expr = self.take_complete_expr();
                    self.stack.complete_expr = Some(match expr {
                        Some(opd) => VdSynExprData::Prefix {
                            opr,
                            opd: self.builder.alloc_expr(opd),
                        },
                        None => VdSynExprData::Err(
                            OriginalVdSynExprError::NoOperandForPrefixOperator { opr }.into(),
                        ),
                    })
                }
                IncompleteVdSynExprData::SeparatedList {
                    separator,
                    mut fragments,
                } => {
                    let expr = self.take_complete_expr();
                    match expr {
                        Some(expr) => match expr.class() {
                            VdSynExprClass::Atom => {
                                match fragments.last().expect("fragments are always non-empty") {
                                    Left(_) => match separator {
                                        VdSeparator::Base(base_separator) => match base_separator {
                                            VdBaseSeparator::Space => {
                                                let expr = self.builder.alloc_expr(expr);
                                                fragments.push(Left(expr));
                                                self.stack.incomplete_exprs.push((
                                                    IncompleteVdSynExprData::SeparatedList {
                                                        separator,
                                                        fragments,
                                                    },
                                                    VdPrecedence::SPACE,
                                                ))
                                            }
                                            VdBaseSeparator::Comma => todo!(),
                                            VdBaseSeparator::Semicolon => todo!(),
                                            VdBaseSeparator::Add => todo!(),
                                            VdBaseSeparator::Mul => todo!(),
                                            VdBaseSeparator::Dot => todo!(),
                                        },
                                        VdSeparator::Composite(composite_separator) => todo!(),
                                    },
                                    Right(separator) => todo!(),
                                }
                            }
                            VdSynExprClass::Prefix => todo!(),
                            VdSynExprClass::Suffix => todo!(),
                            VdSynExprClass::Separator => {
                                use husky_print_utils::p;
                                p!(expr);
                                todo!()
                            }
                        },
                        None => {
                            if separator1 == Some(separator) {
                                use husky_debug_utils::detonate;
                                use husky_print_utils::p;
                                p!(self.show());
                                p!(self.stack);
                                detonate!(100);
                                self.stack.incomplete_exprs.push((
                                    IncompleteVdSynExprData::SeparatedList {
                                        separator,
                                        fragments,
                                    },
                                    precedence,
                                ))
                            } else {
                                self.stack.complete_expr = Some(VdSynExprData::SeparatedList {
                                    separator,
                                    fragments,
                                })
                            }
                        }
                    }
                }
                IncompleteVdSynExprData::Delimited { bra } => todo!(),
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
        self.push_top_syn_expr(None, top_expr)
    }

    pub(super) fn finish_batch(&mut self) -> Option<VdSynExprIdx> {
        assert!(self.stack.incomplete_exprs.len() == 0);
        std::mem::take(&mut self.stack.complete_expr).map(|expr| self.builder.alloc_expr(expr))
    }

    pub(super) fn last_left_delimiter(&self) -> Option<VdBaseLeftDelimiter> {
        todo!()
        // for (unfinished_expr, _) in self.stack.incomplete_exprs.iter().rev() {
        //     match unfinished_expr {
        //         IncompleteVdSynExprData::SeparatedList { bra, .. } => return Some(*bra),
        //         IncompleteVdSynExprData::CallList { .. } => todo!(),
        //         //  return Some(Delimiter::Par),
        //         _ => (),
        //     }
        // }
        // None
    }
}

impl VdSynExprStack {
    pub fn show(&self, arena: VdSynExprArenaRef) -> String {
        use std::fmt::Write;

        let mut s = "Stack { incomplete: [".to_string();
        // Show incomplete expressions with precedence
        for (i, (expr, precedence)) in self.incomplete_exprs.iter().enumerate() {
            if i > 0 {
                s += ", ";
            }
            write!(s, "({}, {})", expr.show(arena), precedence.to_string());
        }
        s += "], complete: ";
        if let Some(expr) = &self.complete_expr {
            s += &expr.show(arena);
        } else {
            s += "None";
        };
        s += " }";
        s
    }
}
