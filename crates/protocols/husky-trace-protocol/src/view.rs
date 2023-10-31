pub mod action;
pub mod data_ref;

use crate::*;
use husky_token_protocol::TokenClass;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewData {
    lines_data: Vec<TraceViewLineData>,
    have_subtraces: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewLineData {
    tokens_data: Vec<TraceViewTokenData>,
}

impl TraceViewData {
    pub fn new(lines: Vec<TraceViewLineData>, have_subtraces: bool) -> Self {
        Self {
            lines_data: lines,
            have_subtraces,
        }
    }

    pub fn lines_data(&self) -> &[TraceViewLineData] {
        self.lines_data.as_ref()
    }

    pub fn have_subtraces(&self) -> bool {
        self.have_subtraces
    }
}

impl TraceViewLineData {
    pub fn new(tokens_data: Vec<TraceViewTokenData>) -> Self {
        Self { tokens_data }
    }

    pub fn tokens_data(&self) -> &[TraceViewTokenData] {
        self.tokens_data.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewTokenData {
    text: String,
    token_class: TokenClass,
    spaces_before: u32,
    associated_trace_id: Option<TraceId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeparationAfter {
    SameLine { spaces: u32 },
    NextLine { indent: u32 },
    Eof,
}

impl TraceViewTokenData {
    pub fn new(
        text: String,
        token_class: TokenClass,
        spaces_before: u32,
        associated_trace: Option<TraceId>,
    ) -> Self {
        Self {
            text,
            token_class,
            spaces_before,
            associated_trace_id: associated_trace,
        }
    }

    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    pub fn spaces_before(&self) -> u32 {
        self.spaces_before
    }

    pub fn token_class(&self) -> TokenClass {
        self.token_class
    }

    pub fn associated_trace_id(&self) -> Option<TraceId> {
        self.associated_trace_id
    }
}
