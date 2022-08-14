use husky_primitive_literal_syntax::PrimitiveLiteralData;
use husky_word::{Keyword, Paradigm};

use super::*;

pub trait AtomParserPattern {
    type Output;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>>;
}
