use husky_entity_route::EntityRoutePtr;

use super::*;

impl<'a> TraceLineBuilder<'a> {
    pub(super) fn gen_mod(&mut self) {
        self.push_token(TraceTokenData {
            kind: TraceTokenKind::Mod,
            value: "mod ".to_string(),
            opt_associated_trace_id: None,
        })
    }

    pub(super) fn gen_ident_token(&mut self, name: &str, opt_associated_trace_id: Option<TraceId>) {
        self.push_token(TraceTokenData {
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
        self.push_token(TraceTokenData {
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
        self.push_token(TraceTokenData {
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
        self.push_token(TraceTokenData {
            kind: TraceTokenKind::Scope,
            value,
            opt_associated_trace_id,
        })
    }

    pub(super) fn gen_assign_token(&mut self) {
        self.push_token(TraceTokenData {
            kind: TraceTokenKind::Special,
            value: " = ".to_string(),
            opt_associated_trace_id: None,
        });
    }

    pub(super) fn gen_literal_token<T: std::fmt::Display>(&mut self, t: T) {
        self.push_token(TraceTokenData {
            kind: TraceTokenKind::Literal,
            value: format!("{}", t),
            opt_associated_trace_id: None,
        });
    }

    pub(super) fn gen_fade_assign_token(&mut self) {
        self.push_token(TraceTokenData {
            kind: TraceTokenKind::Special,
            value: " = ".to_string(),
            opt_associated_trace_id: None,
        })
    }

    pub(super) fn gen_error_token(&mut self, e: &__VMError) {
        self.push_token(TraceTokenData {
            kind: TraceTokenKind::Error,
            value: e.message().to_string(),
            opt_associated_trace_id: None,
        });
    }

    pub(super) fn gen_fade_token(&mut self, value: &str) {
        self.push_token(TraceTokenData {
            kind: TraceTokenKind::Fade,
            value: "???".to_string(),
            opt_associated_trace_id: None,
        })
    }

    pub(super) fn gen_result_token(
        &mut self,
        result: __VMResult<__Register<'static>>,
        intrinsic_ty: EntityRoutePtr,
    ) {
        self.push_token(
            self.trace_time
                .trace_token_from_result(result, intrinsic_ty),
        )
    }

    fn push_token(&mut self, token: TraceTokenData) {
        self.lines.last_mut().unwrap().tokens.push(token)
    }
}
