use std::ops::AddAssign;

use ast::{Expr, ExprError, ExprResult};
use atom::StmtAttr;
use word::WordInterner;

use crate::*;

pub struct Formatter<'a> {
    word_interner: &'a WordInterner,
    arena: &'a ast::ExprArena,
    formatted_text: FormattedText,
}

impl<'a>
    folded::Generator<'_, ExprResult, folded::FoldedList<ExprResult>, Result<String, ExprError>>
    for Formatter<'a>
{
    fn enter_fold(&mut self) {}

    fn exit_fold(&mut self) {}

    fn transform(&mut self, input: &ExprResult) -> Result<String, ExprError> {
        todo!()

        // Ok(input
        //     .map_err(|e| e.clone())?
        //     .as_ref()
        //     .map(|(attr, expr)| self.fmt(&attr, &expr))
        //     .or("".to_string()))
    }

    fn folded_results(&mut self) -> &mut FormattedText {
        &mut self.formatted_text
    }
}

impl<'a> Formatter<'a> {
    fn fmt(&self, attr: &StmtAttr, expr: &Option<Expr>) -> String {
        let mut result = String::new();
        if let Some(keyword) = &attr.keyword {
            result += keyword.code();
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
            ast::ExprKind::Variable(ident) => self
                .word_interner
                .apply(word::Word::Identifier(*ident), |s| result.add_assign(s)),
            ast::ExprKind::Literal(literal) => match literal {
                atom::Literal::I32Literal(i) => result.add_assign(&i.to_string()),
                atom::Literal::F32Literal(f) => result.add_assign(&f.to_string()),
            },
            ast::ExprKind::Bracketed(expr_idx) => {
                result.add_assign("(");
                self.fmt_expr(result, &self.arena[expr_idx]);
                result.add_assign(")");
            }
            ast::ExprKind::Opn { opr: opn, opds } => match opn {
                ast::Opr::Binary(_) => todo!(),
                ast::Opr::Prefix(_) => todo!(),
                ast::Opr::Suffix(_) => todo!(),
                ast::Opr::List(_) => todo!(),
                // ast::Opr::ScopeCall(_) => todo!(),
                // ast::Opr::ValueCall => {
                //     let opds = &self.arena[opds];
                //     self.fmt_expr(result, &opds[0]);
                //     result.add_assign("(");
                //     for i in 1..opds.len() {
                //         if i >= 2 {
                //             result.add_assign(", ")
                //         }
                //         self.fmt_expr(result, &opds[i]);
                //     }
                //     result.add_assign(")");
                // }
                // ast::Opr::MemberCall(_) => todo!(),
                // ast::Opr::Member(_) => todo!(),
                // ast::Opr::Index => todo!(),
                // ast::Opr::Opr(opr) => match opr {},
            },
            ast::ExprKind::Void => result.add_assign("()"),
            ast::ExprKind::Scope(_) => todo!(),
        }
    }
}
