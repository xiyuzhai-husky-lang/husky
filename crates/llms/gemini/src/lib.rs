//! Gemini API client implementation for interacting with Google's Gemini language model.
mod cap;
pub mod client;
pub mod error;
pub mod model;
pub mod raw;
pub mod request;
pub mod response;
pub mod tier;

use self::{
    cap::*,
    error::{GeminiError, GeminiResult},
    raw::*,
};
