use crate::expr::{LeanHirExprArenaRef, LeanHirExprData, LeanHirExprIdx};
use lean_opr::precedence::LeanPrecedenceRange;

pub struct LeanHirExprFormatter<'a> {
    db: &'a ::salsa::Db,
    arena: LeanHirExprArenaRef<'a>,
    line_max_len: usize,
    result: String,
}

impl<'a> LeanHirExprFormatter<'a> {
    pub fn format_expr_ext(&mut self, expr: LeanHirExprIdx) {
        self.format_expr(expr, false, LeanPrecedenceRange::Any);
    }

    fn format_expr(
        &mut self,
        expr: LeanHirExprIdx,
        try_multiline: bool,
        precedence_range: LeanPrecedenceRange,
    ) {
        let needs_bracket = !precedence_range.include(self.arena[expr].precedence());
        if needs_bracket {
            // TODO: consider multiline
            self.result += "(";
        }
        let prev_len = self.result.len();
        self.format_expr_inner(expr, false);
        if try_multiline && !self.check_lines(prev_len) {
            self.result.truncate(prev_len);
            self.format_expr_inner(expr, true);
        }
        if needs_bracket {
            // TODO: consider multiline
            self.result += ")";
        }
    }

    fn format_expr_inner(&mut self, expr: LeanHirExprIdx, multiline: bool) {
        // Lean formatter rule: outer expressions should multiline prior to inner expressions.
        // This ensures that subexpressions only attempt multiline formatting if the parent is already multiline.
        let subexpr_try_multiline = multiline;
        let db = self.db;
        let arena = self.arena;
        match arena[expr] {
            LeanHirExprData::Variable { ident } => {
                if !self.result.ends_with(['(', ' ']) {
                    self.result.push(' ');
                }
                self.result += ident.data(db)
            }
            LeanHirExprData::Prefix { opr, opd } => {
                self.result += opr.fmt_str();
                self.format_expr(opd, subexpr_try_multiline, opr.precedence_range());
            }
            LeanHirExprData::Suffix { opd, opr } => {
                self.format_expr(opd, subexpr_try_multiline, opr.precedence_range());
                self.result += opr.fmt_str();
            }
            LeanHirExprData::Binary { lopd, opr, ropd } => {
                self.format_expr(lopd, subexpr_try_multiline, opr.left_precedence_range());
                if !self.result.ends_with(' ') {
                    self.result.push(' ');
                }
                self.result += opr.fmt_str();
                self.result.push(' ');
                self.format_expr(ropd, subexpr_try_multiline, opr.right_precedence_range());
            }
            LeanHirExprData::Lambda {
                ref parameters,
                body,
            } => {
                self.result.push('λ');
                for (i, param) in parameters.iter().enumerate() {
                    if i > 0 {
                        self.result.push(' ');
                    }
                    self.result += param.ident().data(db);
                    self.result.push_str(" : ");
                    self.format_expr(param.ty(), false, LeanPrecedenceRange::Any);
                }
                self.result += " => ";
                if multiline {
                    self.result.push('\n');
                    self.result.push_str("  "); // Indent the body
                }
                self.format_expr(body, multiline, LeanPrecedenceRange::Any);
            }
            LeanHirExprData::Application {
                function_and_arguments,
            } => {
                for expr in function_and_arguments {
                    self.format_expr(
                        expr,
                        subexpr_try_multiline,
                        LeanPrecedenceRange::APPLICATION_SUBEXPR,
                    );
                }
            }
        }
    }

    fn check_lines(&self, prev_len: usize) -> bool {
        // Find the end of the previous line
        let prev_line_end_offset = self.result[..prev_len]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);

        // Check all lines from the previous line end
        self.result[prev_line_end_offset..]
            .lines()
            .all(|line| line.len() <= self.line_max_len)
    }
}
