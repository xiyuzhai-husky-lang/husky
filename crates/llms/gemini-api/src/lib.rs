//! Gemini API client implementation for interacting with Google's Gemini language model.
pub mod client;
pub mod common;
pub mod error;
pub mod request;
pub mod response;

use self::common::*;
use self::error::{GeminiApiError, GeminiApiResult};
