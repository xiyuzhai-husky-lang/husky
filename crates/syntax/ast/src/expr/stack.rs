use atom::{Bracket, ListEndAttr, ListStartAttr, PrefixOpr, SuffixOpr};
use check_utils::should;
use entity_route::RangedEntityRoute;
use text::{TextPosition, TextRange};
use vm::{BinaryOpr, PrimitiveValue};

use crate::{expr::precedence::Precedence, *};

pub(crate) struct ExprStack<'a> {
    arena: &'a mut RawExprArena,
    oprs: Vec<ExprStackOpr>,
    exprs: Vec<RawExpr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ExprStackOpr {
    precedence: Precedence,
    kind: ExprStackOprKind,
}

impl ExprStackOpr {
    fn binary(opr: BinaryOpr) -> Self {
        let precedence = opr.into();
        Self {
            precedence,
            kind: ExprStackOprKind::Binary(opr),
        }
    }

    fn prefix(prefix: PrefixOpr, start: TextPosition) -> Self {
        Self {
            precedence: Precedence::Prefix,
            kind: ExprStackOprKind::Prefix { prefix, start },
        }
    }

    fn list_item() -> Self {
        Self {
            precedence: Precedence::None,
            kind: ExprStackOprKind::ListItem,
        }
    }

    fn list_start(bra: Bracket, attr: ListStartAttr, start: TextPosition) -> Self {
        Self {
            precedence: Precedence::None,
            kind: ExprStackOprKind::ListStart { bra, attr, start },
        }
    }

    fn lambda_head(
        inputs: Vec<(CustomIdentifier, Option<RangedEntityRoute>)>,
        start: TextPosition,
    ) -> Self {
        Self {
            precedence: Precedence::LambdaHead,
            kind: ExprStackOprKind::LambdaHead { inputs, start },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ExprStackOprKind {
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
    LambdaHead {
        inputs: Vec<(CustomIdentifier, Option<RangedEntityRoute>)>,
        start: TextPosition,
    },
}

impl<'a> std::fmt::Debug for ExprStack<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExprStack")
            .field("oprs", &self.oprs)
            .field("exprs", &self.exprs)
            .finish()
    }
}

impl<'a> ExprStack<'a> {
    pub(crate) fn new(arena: &'a mut RawExprArena) -> Self {
        Self {
            arena,
            oprs: Vec::new(),
            exprs: Vec::new(),
        }
    }

    pub(crate) fn finish(mut self) -> RawExprIdx {
        self.synthesize_all_above(Precedence::None);
        should!(self.exprs.len() == 1);
        self.arena.alloc_one(self.exprs.pop().unwrap())
    }

    pub(crate) fn accept_list_start(
        &mut self,
        bra: Bracket,
        attr: ListStartAttr,
        start: TextPosition,
    ) {
        self.oprs.push(ExprStackOpr::list_start(bra, attr, start));
        match attr {
            ListStartAttr::None => (),
            ListStartAttr::Attach => self.oprs.push(ExprStackOpr::list_item()),
        };
    }

    pub(crate) fn accept_list_item(&mut self) {
        self.synthesize_all_above(Precedence::ListItem);
        self.oprs.push(ExprStackOpr::list_item())
    }

    pub(crate) fn accept_list_end(
        &mut self,
        ket: Bracket,
        attr: ListEndAttr,
        end: TextPosition,
    ) -> AstResult<()> {
        self.synthesize_list(ket, attr, end)
    }

    pub(crate) fn accept_binary(&mut self, binary: BinaryOpr) {
        let stack_opr = ExprStackOpr::binary(binary);
        self.synthesize_all_above(stack_opr.precedence);
        self.oprs.push(stack_opr);
    }

    pub(crate) fn accept_prefix(&mut self, prefix: PrefixOpr, start: TextPosition) {
        self.oprs.push(ExprStackOpr::prefix(prefix, start))
    }

    pub(crate) fn accept_suffix(&mut self, suffix: SuffixOpr, end: TextPosition) {
        self.synthesize_suffix(suffix, end)
    }

    pub(crate) fn accept_atom_expr(&mut self, expr: RawExpr) {
        self.exprs.push(expr);
    }

    pub(crate) fn accept_lambda_head(
        &mut self,
        inputs: Vec<(CustomIdentifier, Option<RangedEntityRoute>)>,
        start: TextPosition,
    ) {
        self.oprs.push(ExprStackOpr::lambda_head(inputs, start))
    }
}

impl<'a> ExprStack<'a> {
    fn top(&self, i: usize) -> &ExprStackOpr {
        &self.oprs[self.oprs.len() - 1 - i]
    }
    fn synthesize_list(
        &mut self,
        ket: Bracket,
        end_attr: ListEndAttr,
        end: TextPosition,
    ) -> AstResult<()> {
        let (start_attr, start, list_len) = {
            let mut i = 0;
            loop {
                if i >= self.oprs.len() {
                    todo!()
                }
                match self.top(i).kind {
                    ExprStackOprKind::ListItem => (),
                    ExprStackOprKind::ListStart { bra, attr, start } => {
                        if ket != bra {
                            err!(
                                format!(
                                    "brackets should match but get bra = {}, ket = {}",
                                    bra.bra_code(),
                                    ket.ket_code(),
                                ),
                                (self.exprs[0].range.start..end).into()
                            )?;
                        };
                        break (attr, start, i);
                    }
                    _ => {
                        err!(
                            format!(
                                "expect {} but got {:?} instead",
                                ket.bra_code(),
                                self.oprs[i]
                            ),
                            (self.exprs[0].range.start..end).into()
                        )?;
                    }
                }
                i += 1;
            }
        };
        self.oprs.truncate(self.oprs.len() - list_len - 1);
        let true_start = match start_attr {
            ListStartAttr::None => start,
            ListStartAttr::Attach => self.exprs[self.exprs.len() - list_len].range.start,
        };
        let opds = self
            .arena
            .alloc(self.exprs[self.exprs.len() - list_len..].into());
        self.exprs.truncate(self.exprs.len() - list_len);
        self.exprs.push(RawExpr::synthesize_list(
            ket,
            start_attr,
            end_attr,
            (true_start..end).into(),
            opds,
        ));
        Ok(())
    }

    fn synthesize_all_above(&mut self, threshold: Precedence) {
        while let Some(stack_opr) = self.oprs.last().map(|opr| opr.clone()) {
            if stack_opr.precedence >= threshold {
                self.oprs.pop();
                match stack_opr.kind {
                    ExprStackOprKind::Binary(binary) => self.synthesize_binary(binary),
                    ExprStackOprKind::Prefix { prefix, start } => {
                        self.synthesize_prefix(prefix, start)
                    }
                    ExprStackOprKind::LambdaHead { inputs, start } => {
                        self.synthesize_lambda(inputs, start)
                    }
                    ExprStackOprKind::ListItem | ExprStackOprKind::ListStart { .. } => panic!(),
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
        if prefix == PrefixOpr::Minus {
            if let RawExprVariant::PrimitiveLiteral(lit) = self.exprs.last().unwrap().kind {
                self.exprs.pop();
                match lit {
                    PrimitiveValue::I32(i) => self.exprs.push(RawExpr {
                        range,
                        kind: RawExprVariant::PrimitiveLiteral(PrimitiveValue::I32(-i)),
                    }),
                    PrimitiveValue::F32(f) => self.exprs.push(RawExpr {
                        range,
                        kind: RawExprVariant::PrimitiveLiteral(PrimitiveValue::F32(-f)),
                    }),
                    PrimitiveValue::Void
                    | PrimitiveValue::B32(_)
                    | PrimitiveValue::Bool(_)
                    | PrimitiveValue::B64(_) => todo!(),
                }
                return;
            }
        }
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
        self.exprs.push(RawExpr {
            range,
            kind: RawExprVariant::Opn { opr: opr, opds },
        });
    }

    fn synthesize_lambda(
        &mut self,
        inputs: Vec<(CustomIdentifier, Option<RangedEntityRoute>)>,
        start: TextPosition,
    ) {
        let range = (start..self.exprs.last().unwrap().range.end).into();
        let lambda_expr = RawExpr {
            range,
            kind: RawExprVariant::Lambda(inputs, self.arena.alloc_one(self.exprs.pop().unwrap())),
        };
        self.exprs.push(lambda_expr);
    }
}
