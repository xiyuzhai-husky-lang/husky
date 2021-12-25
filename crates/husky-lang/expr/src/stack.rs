use atom::{BinaryOpr, Bracket, LambdaHead, ListEndAttr, ListStartAttr, PrefixOpr, SuffixOpr};
use text::{TextPosition, TextRange};

use crate::{error::ExprRule, precedence::Precedence, *};

pub(crate) struct ExprStack<'a> {
    arena: &'a mut ExprArena,
    oprs: Vec<StackOpr>,
    exprs: Vec<Expr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct StackOpr {
    precedence: Precedence,
    kind: StackOprKind,
}

impl StackOpr {
    fn binary(opr: BinaryOpr) -> Self {
        let precedence = opr.into();
        Self {
            precedence: precedence,
            kind: StackOprKind::Binary(opr),
        }
    }

    fn prefix(prefix: PrefixOpr, start: TextPosition) -> Self {
        Self {
            precedence: Precedence::Prefix,
            kind: StackOprKind::Prefix { prefix, start },
        }
    }

    fn list_item() -> Self {
        Self {
            precedence: Precedence::None,
            kind: StackOprKind::ListItem,
        }
    }

    fn list_start(bra: Bracket, attr: ListStartAttr, start: TextPosition) -> Self {
        Self {
            precedence: Precedence::None,
            kind: StackOprKind::ListStart { bra, attr, start },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum StackOprKind {
    Binary(BinaryOpr),
    ListItem,
    Prefix {
        prefix: PrefixOpr,
        start: TextPosition,
    },
    ListStart {
        bra: Bracket,
        attr: ListStartAttr,
        start: TextPosition,
    },
    LambdaHead(LambdaHead, TextPosition),
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
        self.synthesize_all_above(Precedence::None);
        assert!(self.exprs.len() == 1);
        self.exprs.pop().unwrap()
    }

    pub(crate) fn accept_list_start(
        &mut self,
        bra: Bracket,
        attr: ListStartAttr,
        start: TextPosition,
    ) {
        match attr {
            ListStartAttr::None => (),
            ListStartAttr::Attach => self.oprs.push(StackOpr::list_item()),
        };
        self.oprs.push(StackOpr::list_start(bra, attr, start))
    }

    pub(crate) fn accept_list_item(&mut self) {
        self.synthesize_all_above(Precedence::ListItem);
        self.oprs.push(StackOpr::list_item())
    }

    pub(crate) fn accept_list_end(
        &mut self,
        ket: Bracket,
        attr: ListEndAttr,
        end: TextPosition,
    ) -> Result<(), ExprError> {
        self.synthesize_list(ket, attr, end)
    }

    pub(crate) fn accept_binary(&mut self, binary: BinaryOpr) {
        let stack_opr = StackOpr::binary(binary);
        self.synthesize_all_above(stack_opr.precedence);
        self.oprs.push(stack_opr);
    }

    pub(crate) fn accept_prefix(&mut self, prefix: PrefixOpr, start: TextPosition) {
        self.oprs.push(StackOpr::prefix(prefix, start))
    }

    pub(crate) fn accept_suffix(&mut self, suffix: SuffixOpr, end: TextPosition) {
        self.synthesize_suffix(suffix, end)
    }

    pub(crate) fn accept_atom_expr(&mut self, expr: Expr) {
        self.exprs.push(expr);
    }
}

impl<'a> ExprStack<'a> {
    fn synthesize_list(
        &mut self,
        ket: Bracket,
        end_attr: ListEndAttr,
        end: TextPosition,
    ) -> Result<(), ExprError> {
        let (start_attr, start, list_len) = {
            let mut i = 0;
            loop {
                if i >= self.oprs.len() {
                    todo!()
                }
                match self.oprs[i].kind {
                    StackOprKind::ListItem => (),
                    StackOprKind::ListStart { bra, attr, start } => {
                        if ket != bra {
                            return Err(ExprError::new(
                                self.exprs[0].range.start..end,
                                ExprRule::BracketsShouldMatch,
                            ));
                        };
                        break (attr, start, i);
                    }
                    _ => {
                        return Err(ExprError::new(
                            (self.exprs[0].range.start..end).into(),
                            ExprRule::BracketsShouldMatch,
                        ))
                    }
                }
                i += 1;
            }
        };
        self.oprs.truncate(self.oprs.len() - list_len - 1);
        let opds = self
            .arena
            .alloc(self.exprs[self.exprs.len() - list_len..].into());
        self.exprs.truncate(self.exprs.len() - list_len);
        self.exprs.push(Expr::list(
            ket,
            start_attr,
            end_attr,
            (start..end).into(),
            opds,
        ));
        Ok(())
    }

    fn synthesize_all_above(&mut self, threshold: Precedence) {
        while let Some(stack_opr) = self.oprs.last().map(|opr| opr.clone()) {
            if stack_opr.precedence >= threshold {
                self.oprs.pop();
                match stack_opr.kind {
                    StackOprKind::Binary(binary) => self.synthesize_binary(binary),
                    StackOprKind::Prefix { prefix, start } => self.synthesize_prefix(prefix, start),
                    _ => panic!(),
                }
            } else {
                return;
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
            kind: ExprKind::Opn { opr: opr, opds },
        });
    }
}
