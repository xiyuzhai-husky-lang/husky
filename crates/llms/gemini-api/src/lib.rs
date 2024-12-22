//! Gemini API client implementation for interacting with Google's Gemini language model.
mod cap;
pub mod client;
pub mod common;
pub mod error;
pub mod request;
pub mod response;

use self::{
    cap::*,
    common::*,
    error::{GeminiError, GeminiResult},
};
