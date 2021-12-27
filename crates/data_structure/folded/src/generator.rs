use crate::*;

pub trait Generator<'a, Input: 'a, Storage, Output>
where
    Storage: FoldedStorage<Input>,
    Input: ?Sized,
{
    fn enter_fold(&mut self);
    fn exit_fold(&mut self, idx: folded_list::FoldedIdx<Output>);
    fn transform(&mut self, input: &Input) -> Output;
    fn folded_results_mut(&mut self) -> &mut FoldedList<Output>;

    fn transform_all(&mut self, mut iter: FoldedIter<'a, Input, Storage>) {
        while let Some((_, value, children)) = iter.next() {
            // parse current
            self.enter_fold();
            let parse_result = self.transform(value);
            let idx = self.folded_results_mut().append(parse_result, iter.next);
            // parse children
            if let Some(children) = children {
                self.transform_all(children);
            }
            self.exit_fold(idx);
        }
    }
}
