use crate::*;

pub trait Parser<'a, Input: 'a, Storage, ParseResult, This>
where
    This: Parser<'a, Input, Storage, ParseResult, This>,
    Storage: FoldedStorage<Input, Storage>,
    Input: ?Sized,
{
    fn enter_fold(&mut self);
    fn exit_fold(&mut self);
    fn parse(&mut self, value: &Input) -> ParseResult;
    fn folded_results(&mut self) -> &mut FoldedList<ParseResult>;

    fn parse_all(&mut self, mut iter: FoldedIter<'a, Input, Storage>) {
        let mut child_iter = iter.children();
        while let Some((_, value)) = iter.next() {
            // parse current
            let parse_result = self.parse(value);
            self.folded_results().append(parse_result, iter.next);
            // parse children
            self.enter_fold();
            self.parse_all(child_iter);
            self.exit_fold();
            // reset child iter
            child_iter = iter.children();
        }
    }
}
