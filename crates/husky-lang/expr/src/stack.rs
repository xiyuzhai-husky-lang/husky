use atom::{BinaryOpr, Bracket, Opr, PrefixOpr, SuffixOpr};
use text::TextPosition;

use crate::{error::ExprRule, kind::Opn, precedence::Precedence, *};

pub(crate) struct ExprStack<'a> {
    arena: &'a mut ExprArena,
    oprs: Vec<StackOpr>,
    exprs: Vec<Expr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum StackOpr {
    Binary {
        binary: BinaryOpr,
        precedence: Precedence,
    },
    Join,
    Prefix {
        prefix: PrefixOpr,
        precedence: Precedence,
        start: TextPosition,
    },
    Bra {
        bra: Bracket,
        start: TextPosition,
    },
}

impl<'a> ExprStack<'a> {
    pub(crate) fn new(arena: &'a mut ExprArena) -> Self {
        Self {
            arena,
            oprs: Vec::new(),
            exprs: Vec::new(),
        }
    }

    pub(crate) fn finish(mut self) -> Expr {
        assert!(self.exprs.len() == 1);
        self.exprs.pop().unwrap()
    }

    pub(crate) fn accept_opr(&mut self, opr: Opr, range: TextRange) {
        let precedence: Precedence = opr.into();
        match opr {
            Opr::Binary(binary) => {
                self.synthesize_all_above(precedence);
                self.oprs.push(StackOpr::Binary { binary, precedence });
            }
            Opr::Join => {
                self.synthesize_all_above(precedence);
                self.oprs.push(StackOpr::Join)
            }
            Opr::Prefix(prefix) => self.oprs.push(StackOpr::Prefix {
                prefix,
                precedence,
                start: range.start,
            }),
            Opr::Bra(bra) => self.oprs.push(StackOpr::Bra {
                bra,
                start: range.start,
            }),
            Opr::Suffix(suffix) => {
                self.synthesize_all_above(precedence);
                self.synthesize_suffix(suffix, range.end)
            }
            Opr::Ket(_) => todo!(),
        }
    }

    pub(crate) fn accept_empty_parenthesis(&mut self, range: TextRange) {
        self.accept_bracketed_exprs(Vec::new(), range);
    }

    pub(crate) fn accept_atom_expr(&mut self, expr: Expr) {
        self.exprs.push(expr);
    }

    fn accept_bracketed_exprs(
        &mut self,
        mut bracketed_exprs: Vec<Expr>,
        range: TextRange,
    ) -> Result<(), ExprError> {
        match self.oprs.last().map(|opr| opr.clone()) {
            Some(StackOpr::Binary {
                binary: BinaryOpr::Call,
                precedence,
            }) => {
                let expr = self.exprs.pop().unwrap();
                match expr.kind {
                    ExprKind::Scope(scope_id) => Ok(self.exprs.push(Expr::scope_call(
                        (expr.range..range).into(),
                        scope_id,
                        self.arena.alloc(bracketed_exprs),
                    ))),
                    _ => {
                        let range = (expr.range..range).into();
                        bracketed_exprs.insert(0, expr);
                        self.exprs
                            .push(Expr::value_call(range, self.arena.alloc(bracketed_exprs)));
                        Ok(())
                    }
                }
            }
            Some(StackOpr::Binary {
                binary: BinaryOpr::Index,
                precedence,
            }) => {
                let expr = self.exprs.pop().unwrap();
                let range = (expr.range..range).into();
                bracketed_exprs.insert(0, expr);
                self.exprs
                    .push(Expr::index(range, self.arena.alloc(bracketed_exprs)));
                Ok(())
            }
            _ => {
                if bracketed_exprs.len() > 1 {
                    Err(ExprError::new(
                        range,
                        ExprRule::BracketedExprCommaListShouldBeEitherCalledOrIndexed,
                    ))
                } else if bracketed_exprs.len() == 0 {
                    self.exprs.push(Expr {
                        range,
                        kind: ExprKind::Void,
                    });
                    Ok(())
                } else {
                    self.exprs.push(Expr {
                        range,
                        kind: ExprKind::Bracketed(self.arena.alloc(bracketed_exprs).start),
                    });
                    Ok(())
                }
            }
        }
    }
}

impl<'a> ExprStack<'a> {
    fn synthesize_bracketed_exprs(
        &mut self,
        ket: Bracket,
        range: TextRange,
    ) -> Result<(), ExprError> {
        let mut bracketed_exprs = Vec::new();
        self.synthesize_all_above(Precedence::AboveJoin);
        while let Some(stack_opr) = self.oprs.pop() {
            match stack_opr {
                StackOpr::Binary { .. } | StackOpr::Prefix { .. } => panic!(),
                StackOpr::Join => self.synthesize_all_above(Precedence::AboveJoin),
                StackOpr::Bra { bra, start } => {
                    if (bra != ket) {
                        return Err(ExprError::new(
                            (start..range.end).into(),
                            ExprRule::BracketsShouldMatch,
                        ));
                    } else {
                        return self
                            .accept_bracketed_exprs(bracketed_exprs, (start..range.end).into());
                    }
                }
            }
        }
        Err(ExprError::new(range, ExprRule::BracketsShouldMatch))
    }

    fn synthesize_all_above(&mut self, threshold: Precedence) {
        while let Some(stack_opr) = self.oprs.last().map(|opr| opr.clone()) {
            match stack_opr {
                StackOpr::Binary { precedence, binary } => {
                    self.oprs.pop();
                    if precedence >= threshold {
                        self.synthesize_binary(binary)
                    } else {
                        return;
                    }
                }
                StackOpr::Prefix {
                    precedence,
                    prefix,
                    start,
                } => {
                    self.oprs.pop();
                    if precedence >= threshold {
                        self.synthesize_prefix(prefix, start)
                    } else {
                        return;
                    }
                }
                StackOpr::Join | StackOpr::Bra { .. } => break,
            }
        }
    }

    fn synthesize_binary(&mut self, binary: BinaryOpr) {
        let len = self.exprs.len();
        let range = (self.exprs[len - 2].range.start..self.exprs[len - 1].range.end).into();
        self.synthesize_opr(binary.into(), 2, range)
    }

    fn synthesize_prefix(&mut self, prefix: PrefixOpr, start: TextPosition) {
        let range = (start..self.exprs.last().unwrap().range.end).into();
        self.synthesize_opr(prefix.into(), 1, range)
    }

    fn synthesize_suffix(&mut self, suffix: SuffixOpr, end: TextPosition) {
        let range = (self.exprs.last().unwrap().range.start..end).into();
        self.synthesize_opr(suffix.into(), 1, range)
    }

    fn synthesize_opr(&mut self, opr: Opr, n_opds: usize, range: TextRange) {
        let len = self.exprs.len();
        let opds = self.arena.alloc(self.exprs[(len - n_opds)..len].into());
        self.exprs.truncate(len - n_opds);
        self.exprs.push(Expr {
            range,
            kind: ExprKind::Opn {
                opn: opr.into(),
                opds,
            },
        });
    }
}
