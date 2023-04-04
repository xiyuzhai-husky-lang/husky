#![feature(trait_upcasting)]
mod local_term;

use either::*;
use husky_entity_path::*;
use husky_expr::*;
use husky_term::*;
use husky_term_prelude::*;
use salsa::DebugWithDb as _;
use smallvec::*;
