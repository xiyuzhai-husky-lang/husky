use super::*;

impl<'a> TraceLineBuilder<'a> {
    pub(super) fn gen_ident_token(&mut self, name: &str, opt_associated_trace_id: Option<TraceId>) {
        self.gen_token(TraceTokenData {
            kind: TraceTokenKind::Ident,
            value: name.to_string(),
            opt_associated_trace_id,
        })
    }

    pub(super) fn gen_keyword_token(
        &mut self,
        name: &str,
        opt_associated_trace_id: Option<TraceId>,
    ) {
        self.gen_token(TraceTokenData {
            kind: TraceTokenKind::Keyword,
            value: name.to_string(),
            opt_associated_trace_id,
        })
    }

    pub(super) fn gen_special_token(
        &mut self,
        name: &str,
        opt_associated_trace_id: Option<TraceId>,
    ) {
        self.gen_token(TraceTokenData {
            kind: TraceTokenKind::Special,
            value: name.to_string(),
            opt_associated_trace_id,
        })
    }

    pub(super) fn gen_route_token(
        &mut self,
        value: String,
        opt_associated_trace_id: Option<TraceId>,
    ) {
        self.gen_token(TraceTokenData {
            kind: TraceTokenKind::Scope,
            value,
            opt_associated_trace_id,
        })
    }

    pub(super) fn gen_assign_token(&mut self) {
        self.gen_token(TraceTokenData {
            kind: TraceTokenKind::Special,
            value: " = ".to_string(),
            opt_associated_trace_id: None,
        });
    }

    pub(super) fn gen_literal_token<T: std::fmt::Display>(&mut self, t: T) {
        self.gen_token(TraceTokenData {
            kind: TraceTokenKind::Literal,
            value: format!("{}", t),
            opt_associated_trace_id: None,
        });
    }

    pub(super) fn gen_fade_assign(&mut self) {
        self.gen_token(TraceTokenData {
            kind: TraceTokenKind::Special,
            value: " = ".to_string(),
            opt_associated_trace_id: None,
        })
    }

    fn gen_token(&mut self, token: TraceTokenData) {
        self.lines.last_mut().unwrap().tokens.push(token)
    }
}
