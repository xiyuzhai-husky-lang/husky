//! this crate renders the trace doc, which displays the traces and their peripheries
#![feature(let_chains)]
pub mod doc;
pub mod helpers;
pub mod settings;
mod view;

use self::settings::*;
use husky_trace_protocol::view::action::TraceViewAction;
use serde::{Deserialize, Serialize};
