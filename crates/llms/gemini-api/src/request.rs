use crate::common::Content;
use serde::Serialize;

#[derive(Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
}
