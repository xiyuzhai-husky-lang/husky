use super::*;

pub trait AtomParserPattern: std::fmt::Display {
    type Output;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>>;
}
