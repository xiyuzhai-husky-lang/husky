use check_utils::should;
use entity_route::{GenericArgument, RangedEntityRoute};
use text::RangedCustomIdentifier;
use text::{TextPosition, TextRange};
use vm::*;

use crate::{expr::precedence::Precedence, *};

pub(crate) struct ExprStack<'a> {
    arena: &'a mut RawExprArena,
    oprs: Vec<ExprStackOpr>,
    exprs: Vec<RawExpr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ExprStackOpr {
    precedence: Precedence,
    variant: ExprStackOprVariant,
}

impl ExprStackOpr {
    fn binary(opr: BinaryOpr) -> Self {
        let precedence = opr.into();
        Self {
            precedence,
            variant: ExprStackOprVariant::Binary(opr),
        }
    }

    fn prefix(prefix: PrefixOpr, start: TextPosition) -> Self {
        Self {
            precedence: Precedence::Prefix,
            variant: ExprStackOprVariant::Prefix { prefix, start },
        }
    }

    fn list_item(position: TextPosition) -> Self {
        Self {
            precedence: Precedence::None,
            variant: ExprStackOprVariant::ListItem(position),
        }
    }

    fn list_start(
        bra: Bracket,
        attr: ListStartAttr,
        start: TextPosition,
        generic_arguments: Vec<GenericArgument>,
    ) -> Self {
        Self {
            precedence: Precedence::None,
            variant: ExprStackOprVariant::ListStart { bra, attr, start },
        }
    }

    fn lambda_head(
        inputs: Vec<(RangedCustomIdentifier, Option<RangedEntityRoute>)>,
        start: TextPosition,
    ) -> Self {
        Self {
            precedence: Precedence::LambdaHead,
            variant: ExprStackOprVariant::LambdaHead { inputs, start },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ExprStackOprVariant {
    Binary(BinaryOpr),
    ListItem(TextPosition),
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
        inputs: Vec<(RangedCustomIdentifier, Option<RangedEntityRoute>)>,
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

    pub(crate) fn finish(mut self) -> AstResult<RawExprIdx> {
        self.synthesize_all_above(Precedence::None)?;
        should!(self.exprs.len() == 1);
        Ok(self.arena.alloc_one(self.exprs.pop().unwrap()))
    }

    pub(crate) fn accept_list_start(
        &mut self,
        bra: Bracket,
        attr: ListStartAttr,
        start: TextPosition,
        generic_arguments: Vec<GenericArgument>,
    ) {
        let attached = attr.attached();
        self.oprs.push(ExprStackOpr::list_start(
            bra,
            attr,
            start,
            generic_arguments,
        ));
        if attached {
            self.oprs.push(ExprStackOpr::list_item(start))
        }
    }

    pub(crate) fn accept_list_item(&mut self, position: TextPosition) -> AstResult<()> {
        self.synthesize_all_above(Precedence::ListItem)?;
        self.oprs.push(ExprStackOpr::list_item(position));
        Ok(())
    }

    pub(crate) fn accept_list_end(
        &mut self,
        ket: Bracket,
        attr: ListEndAttr,
        end: TextPosition,
    ) -> AstResult<()> {
        self.synthesize_list(ket, attr, end)
    }

    pub(crate) fn accept_binary(&mut self, binary: BinaryOpr) -> AstResult<()> {
        let stack_opr = ExprStackOpr::binary(binary);
        self.synthesize_all_above(stack_opr.precedence)?;
        self.oprs.push(stack_opr);
        Ok(())
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
        inputs: Vec<(RangedCustomIdentifier, Option<RangedEntityRoute>)>,
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
        let original_number_of_oprs = self.oprs.len();
        let (start_attr, start) = {
            loop {
                match self.oprs.pop() {
                    Some(expr_stack_opr) => match expr_stack_opr.variant {
                        ExprStackOprVariant::ListItem(_) => (),
                        ExprStackOprVariant::ListStart { bra, attr, start } => {
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
                            break (attr, start);
                        }
                        _ => {
                            err!(
                                format!(
                                    "expect {} but got {:?} instead",
                                    ket.bra_code(),
                                    expr_stack_opr
                                ),
                                (self.exprs[0].range.start..end).into()
                            )?;
                        }
                    },
                    None => {
                        err!(
                            format!("can't match ket `{}`", ket.ket_code(),),
                            ((end.to_left(1))..end).into()
                        )?;
                    }
                }
            }
        };
        let list_len = original_number_of_oprs - self.oprs.len() - 1;
        let true_start = match start_attr.attached() {
            true => self.exprs[self.exprs.len() - list_len].range.start,
            false => start,
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
        )?);
        Ok(())
    }

    fn synthesize_all_above(&mut self, threshold: Precedence) -> AstResult<()> {
        while let Some(stack_opr) = self.oprs.last() {
            if stack_opr.precedence >= threshold {
                let stack_opr = self.oprs.pop().unwrap();
                match stack_opr.variant {
                    ExprStackOprVariant::Binary(binary) => self.synthesize_binary(binary),
                    ExprStackOprVariant::Prefix { prefix, start } => {
                        self.synthesize_prefix(prefix, start)
                    }
                    ExprStackOprVariant::LambdaHead { inputs, start } => {
                        self.synthesize_lambda(inputs, start)
                    }
                    ExprStackOprVariant::ListItem(position) => {
                        let (bra, start) = loop {
                            if let Some(opr) = self.oprs.pop() {
                                match opr.variant {
                                    ExprStackOprVariant::ListStart { bra, start, .. } => {
                                        break (bra, start)
                                    }
                                    _ => (),
                                }
                            } else {
                                return err!(
                                    format!("improper use of `,`"),
                                    (position..(position.to_right(1))).into()
                                );
                            }
                        };
                        return Err(AstError {
                            variant: AstErrorVariant::Original {
                                message: format!("expect a matching `{}`", bra.ket_code()),
                                range: (start..(start.to_right(1))).into(),
                            },
                            dev_src: dev_src!(),
                        });
                    }
                    ExprStackOprVariant::ListStart { bra, start, .. } => {
                        return Err(AstError {
                            variant: AstErrorVariant::Original {
                                message: format!("expect a matching `{}`", bra.ket_code()),
                                range: (start..(start.to_right(1))).into(),
                            },
                            dev_src: dev_src!(),
                        })
                    }
                }
            } else {
                return Ok(());
            }
        }
        Ok(())
    }

    fn synthesize_binary(&mut self, binary: BinaryOpr) {
        let len = self.exprs.len();
        let range = (self.exprs[len - 2].range.start..self.exprs[len - 1].range.end).into();
        self.synthesize_opr(binary.into(), Vec::new(), 2, range)
    }

    fn synthesize_prefix(&mut self, prefix: PrefixOpr, start: TextPosition) {
        let range = (start..self.exprs.last().unwrap().range.end).into();
        if prefix == PrefixOpr::Minus {
            if let RawExprVariant::CopyableLiteral(lit) = self.exprs.last().unwrap().variant {
                self.exprs.pop();
                match lit {
                    CopyableValue::I32(i) => self.exprs.push(RawExpr {
                        range,
                        variant: RawExprVariant::CopyableLiteral(CopyableValue::I32(-i)),
                    }),
                    CopyableValue::F32(f) => self.exprs.push(RawExpr {
                        range,
                        variant: RawExprVariant::CopyableLiteral(CopyableValue::F32(-f)),
                    }),
                    CopyableValue::Void(_)
                    | CopyableValue::B32(_)
                    | CopyableValue::Bool(_)
                    | CopyableValue::B64(_) => todo!(),
                    CopyableValue::EnumKind(_) => todo!(),
                }
                return;
            }
        }
        self.synthesize_opr(prefix.into(), Vec::new(), 1, range)
    }

    fn synthesize_suffix(&mut self, suffix: SuffixOpr, end: TextPosition) {
        let range = (self.exprs.last().unwrap().range.start..end).into();
        self.synthesize_opr(suffix.into(), Vec::new(), 1, range)
    }

    fn synthesize_opr(
        &mut self,
        opr: Opr,
        generic_arguments: Vec<GenericArgument>,
        n_opds: usize,
        range: TextRange,
    ) {
        let len = self.exprs.len();
        let opds = self.arena.alloc(self.exprs[(len - n_opds)..len].into());
        self.exprs.truncate(len - n_opds);
        self.exprs.push(RawExpr {
            range,
            variant: RawExprVariant::Opn { opr, opds },
        });
    }

    fn synthesize_lambda(
        &mut self,
        inputs: Vec<(RangedCustomIdentifier, Option<RangedEntityRoute>)>,
        start: TextPosition,
    ) {
        let range = (start..self.exprs.last().unwrap().range.end).into();
        let lambda_expr = RawExpr {
            range,
            variant: RawExprVariant::Lambda(
                inputs,
                self.arena.alloc_one(self.exprs.pop().unwrap()),
            ),
        };
        self.exprs.push(lambda_expr);
    }
}
