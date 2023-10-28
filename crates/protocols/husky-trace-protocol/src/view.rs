pub mod action;
pub mod data_ref;

use crate::*;
use husky_token_protocol::TokenClass;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewData {
    tokens_data: Vec<TraceViewTokenData>,
}

impl TraceViewData {
    pub fn new(tokens_data: Vec<TraceViewTokenData>) -> Self {
        Self { tokens_data }
    }

    #[cfg(feature = "mock")]
    pub fn new_mock(tokens_data: impl IntoIterator<Item = (&'static str, TokenClass)>) -> Self {
        Self {
            tokens_data: tokens_data
                .into_iter()
                .map(|(text, token_class)| {
                    TraceViewTokenData::new(
                        text.to_string(),
                        token_class,
                        SeparationAfter::SameLine { spaces: 1 },
                        false,
                    )
                })
                .collect(),
        }
    }

    pub fn tokens_data(&self) -> &[TraceViewTokenData] {
        self.tokens_data.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceViewTokenData {
    text: String,
    token_class: TokenClass,
    separation_after: SeparationAfter,
    associated: bool,
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
        separation_after: SeparationAfter,
        associated: bool,
    ) -> Self {
        Self {
            text,
            token_class,
            separation_after,
            associated,
        }
    }

    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    pub fn separation_after(&self) -> SeparationAfter {
        self.separation_after
    }

    pub fn token_class(&self) -> TokenClass {
        self.token_class
    }
}
