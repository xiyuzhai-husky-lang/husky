use crate::*;

pub trait Transformer<'a, Input: 'a, Storage, Output, This>
where
    This: Transformer<'a, Input, Storage, Output, This>,
    Storage: FoldedStorage<Input, Storage>,
    Input: ?Sized,
{
    fn enter_fold(&mut self);
    fn exit_fold(&mut self);
    fn transform(&mut self, input: &Input) -> Output;
    fn folded_results(&mut self) -> &mut FoldedList<Output>;

    fn convert_all(&mut self, mut iter: FoldedIter<'a, Input, Storage>) {
        let mut child_iter = iter.children();
        while let Some((_, value)) = iter.next() {
            // parse current
            let parse_result = self.transform(value);
            self.folded_results().append(parse_result, iter.next);
            // parse children
            self.enter_fold();
            self.convert_all(child_iter);
            self.exit_fold();
            // reset child iter
            child_iter = iter.children();
        }
    }
}
