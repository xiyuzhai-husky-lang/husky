use crate::*;

pub trait Parser<'a, Input: 'a, Storage, ParseResult, This>
where
    This: Parser<'a, Input, Storage, ParseResult, This>,
    Storage: FoldedStorage<Input, Storage>,
    Input: ?Sized,
{
    fn enter_fold(&mut self);
    fn exit_fold(&mut self);
    fn push(&mut self, result: ParseResult);
    fn new() -> This;
    fn parse_value(&mut self, value: &Input) -> ParseResult;

    fn parse_recursive(
        &mut self,
        mut iter: FoldedIter<'a, Input, Storage>,
        results: &mut FoldedList<ParseResult>,
    ) {
        let mut child_iter = iter.children();
        while let Some((_, value)) = iter.next() {
            // parse current
            results.append(self.parse_value(value), iter.next);
            // parse children
            self.enter_fold();
            self.parse_recursive(child_iter, results);
            self.exit_fold();
            // reset child iter
            child_iter = iter.children();
        }
    }

    fn parse(iter: FoldedIter<'a, Input, Storage>) -> FoldedList<ParseResult> {
        let mut parser = Self::new();
        let mut results = FoldedList::new();
        parser.parse_recursive(iter, &mut results);
        results
    }
}
