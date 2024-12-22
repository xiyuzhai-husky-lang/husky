//! Gemini API client implementation for interacting with Google's Gemini language model.
mod cap;
pub mod client;
pub mod error;
pub mod raw;
pub mod request;
pub mod response;

use self::{
    cap::*,
    error::{GeminiError, GeminiResult},
    raw::*,
};
