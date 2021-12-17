use crate::*;

pub trait Parser<'a, Key, Value: 'a, FoldedThis, ParseResult, This>
where
    This: Parser<'a, Key, Value, FoldedThis, ParseResult, This>,
    FoldedThis: Folded<Key, Value, FoldedThis>,
    Value: ?Sized,
{
    fn enter_fold(&mut self);
    fn exit_fold(&mut self);
    fn push(&mut self, result: ParseResult);
    fn new() -> This;
    fn parse_value(&mut self, value: &Value) -> ParseResult;

    fn parse(mut iter: FoldedIter<'a, Key, Value, FoldedThis>) -> Vec<ParseResult> {
        let mut parser = Self::new();
        let mut child_iter = iter.children();
        let mut result = Vec::new();
        while let Some((_, value)) = iter.next() {
            // parse current
            result.push(parser.parse_value(value));
            // parse children
            parser.enter_fold();
            result.extend(Self::parse(child_iter));
            parser.exit_fold();
            // reset child iter
            child_iter = iter.children();
        }
        result
    }
}
