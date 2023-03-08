mod comment;
mod hex;
mod iter;
mod keylike;
mod string;
mod whitespace;

pub(crate) use iter::*;

use crate::*;
use husky_text_span::DocumentSpan;
use husky_word::WordDb;
use keylike::is_keylike;
use std::sync::Arc;
