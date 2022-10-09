mod ident;
mod opr;
mod special;
mod sporadic;

use std::sync::Arc;

use husky_print_utils::ep;
use husky_text::{Column, Row, TextPosition};
use husky_word::new_word_itr;

use crate::*;

fn tokenize_debug(text: &str) -> String {
    format!("{:#?}", new_word_itr().tokenize(text))
}
