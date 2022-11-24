use husky_term::Term;
use husky_text::{HasTextRange, TextPosition};

use super::*;

impl<'a> TraceLineGenerator<'a> {
    pub(super) fn gen_mod(&mut self) {
        self.push_token(
            TraceTokenData {
                kind: TraceTokenKind::Mod,
                value: "mod ".to_string(),
                opt_associated_trace_id: None,
            },
            None,
        )
    }

    pub(super) fn render_ident_token(
        &mut self,
        name: &str,
        opt_associated_trace_id: Option<TraceId>,
        opt_pos: Option<TextPosition>,
    ) {
        self.push_token(
            TraceTokenData {
                kind: TraceTokenKind::Ident,
                value: name.to_string(),
                opt_associated_trace_id,
            },
            opt_pos,
        )
    }

    pub(super) fn render_keyword_token(
        &mut self,
        name: &str,
        opt_associated_trace_id: Option<TraceId>,
        opt_pos: Option<TextPosition>,
    ) {
        self.push_token(
            TraceTokenData {
                kind: TraceTokenKind::Keyword,
                value: name.to_string(),
                opt_associated_trace_id,
            },
            opt_pos,
        )
    }

    pub(super) fn render_special_token(
        &mut self,
        name: &str,
        opt_associated_trace_id: Option<TraceId>,
        opt_pos: Option<TextPosition>,
    ) {
        self.push_token(
            TraceTokenData {
                kind: TraceTokenKind::Special,
                value: name.to_string(),
                opt_associated_trace_id,
            },
            opt_pos,
        )
    }

    pub(super) fn gen_route_token(
        &mut self,
        value: String,
        opt_associated_trace_id: Option<TraceId>,
        opt_pos: Option<TextPosition>,
    ) {
        self.push_token(
            TraceTokenData {
                kind: TraceTokenKind::Scope,
                value,
                opt_associated_trace_id,
            },
            opt_pos,
        )
    }

    pub(super) fn gen_assign_token(&mut self) {
        self.push_token(
            TraceTokenData {
                kind: TraceTokenKind::Special,
                value: " = ".to_string(),
                opt_associated_trace_id: None,
            },
            None,
        );
    }

    pub(super) fn gen_literal_token<T: std::fmt::Display>(
        &mut self,
        t: T,
        opt_pos: Option<TextPosition>,
    ) {
        self.push_token(
            TraceTokenData {
                kind: TraceTokenKind::Literal,
                value: format!("{}", t),
                opt_associated_trace_id: None,
            },
            opt_pos,
        );
    }

    pub(super) fn gen_fade_assign_token(&mut self) {
        self.push_token(
            TraceTokenData {
                kind: TraceTokenKind::Special,
                value: " = ".to_string(),
                opt_associated_trace_id: None,
            },
            None,
        )
    }

    pub(super) fn gen_error_token(&mut self, e: &__VMError, opt_pos: Option<TextPosition>) {
        self.push_token(
            TraceTokenData {
                kind: TraceTokenKind::Error,
                value: e.message().to_string(),
                opt_associated_trace_id: None,
            },
            opt_pos,
        );
    }

    pub(super) fn gen_fade_token(&mut self, _value: &str, opt_pos: Option<TextPosition>) {
        self.push_token(
            TraceTokenData {
                kind: TraceTokenKind::Fade,
                value: "???".to_string(),
                opt_associated_trace_id: None,
            },
            opt_pos,
        )
    }

    pub(super) fn gen_result_token(
        &mut self,
        result: __VMResult<__Register<'static>>,
        intrinsic_ty: Term,
        opt_pos: Option<TextPosition>,
    ) {
        self.push_token(
            self.devtime.trace_token_from_result(result, intrinsic_ty),
            opt_pos,
        )
    }

    fn push_token(&mut self, token: TraceTokenData, opt_pos: Option<TextPosition>) {
        if let Some(pos) = opt_pos {
            if let Some(cur_row) = self.opt_cur_row {
                assert!(pos.row >= cur_row);
                if pos.row > cur_row {
                    self.opt_cur_row = Some(pos.row);
                    self.lines.push(TraceLineData {
                        indent: pos.col.0.try_into().unwrap(),
                        tokens: vec![],
                        idx: self.lines.len(),
                    })
                }
            } else {
                self.opt_cur_row = Some(pos.row)
            }
        }
        self.lines.last_mut().unwrap().tokens.push(token)
    }
}
