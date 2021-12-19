use std::ops::AddAssign;

use atom::GroupAttr;
use expr::{Expr, ExprError, ExprResult};
use word::WordInterner;

use crate::*;

pub struct Formatter<'a> {
    word_interner: &'a WordInterner,
    arena: &'a expr::ExprArena,
    formatted_text: FormattedText,
}

impl<'a>
    folded::Transformer<
        '_,
        ExprResult,
        folded::FoldedList<ExprResult>,
        Result<String, ExprError>,
        Formatter<'a>,
    > for Formatter<'a>
{
    fn enter_fold(&mut self) {}

    fn exit_fold(&mut self) {}

    fn transform(&mut self, input: &ExprResult) -> Result<String, ExprError> {
        input
            .as_ref()
            .map(|(attr, expr)| self.fmt(&attr, &expr))
            .map_err(|e| *e)
    }

    fn folded_results(&mut self) -> &mut FormattedText {
        &mut self.formatted_text
    }
}

impl<'a> Formatter<'a> {
    fn fmt(&self, attr: &GroupAttr, expr: &Option<Expr>) -> String {
        let mut result = String::new();
        if let Some(keyword) = &attr.keyword {
            self.word_interner
                .apply(word::Word::Keyword(*keyword), |s| result += s);
        }
        if let Some(expr) = expr {
            result += " ";
            self.fmt_expr(&mut result, expr);
        }
        if attr.is_head {
            result += ":";
        }
        result
    }

    fn fmt_expr(&self, result: &mut String, expr: &Expr) {
        match &expr.kind {
            expr::ExprKind::Variable(ident) => self
                .word_interner
                .apply(word::Word::Identifier(*ident), |s| result.add_assign(s)),
            expr::ExprKind::Literal(literal) => match literal {
                atom::Literal::I32Literal(i) => result.add_assign(&i.to_string()),
                atom::Literal::F32Literal(f) => result.add_assign(&f.to_string()),
            },
            expr::ExprKind::Bracketed(expr_idx) => {
                result.add_assign("(");
                self.fmt_expr(result, &self.arena[expr_idx]);
                result.add_assign(")");
            }
            expr::ExprKind::Opn { opn, opds } => match opn {
                expr::Opn::ScopeCall(_) => todo!(),
                expr::Opn::ValueCall => {
                    let opds = &self.arena[opds];
                    self.fmt_expr(result, &opds[0]);
                    result.add_assign("(");
                    for i in 1..opds.len() {
                        if i >= 2 {
                            result.add_assign(", ")
                        }
                        self.fmt_expr(result, &opds[i]);
                    }
                    result.add_assign(")");
                }
                expr::Opn::MemberCall(_) => todo!(),
                expr::Opn::Member(_) => todo!(),
                expr::Opn::Index => todo!(),
                expr::Opn::Opr(opr) => match opr {
                    atom::Opr::Binary(_) => todo!(),
                    atom::Opr::Join => todo!(),
                    atom::Opr::Prefix(_) => todo!(),
                    atom::Opr::Suffix(_) => todo!(),
                    atom::Opr::Bra(_) => todo!(),
                    atom::Opr::Ket(_) => todo!(),
                },
            },
            expr::ExprKind::Void => result.add_assign("()"),
            expr::ExprKind::Scope(_) => todo!(),
        }
    }
}
