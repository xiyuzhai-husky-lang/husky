use common::repeat_less_than;

use crate::*;

pub trait Transformer<Input, Storage, Output>
where
    Storage: FoldedContainer<Input>,
    Input: ?Sized,
{
    fn enter(&mut self);
    fn exit(&mut self);
    fn post_exit(&mut self, idx: folded_list::FoldedIdx<Output>);
    fn transform(&mut self, indent: Indent, input: &Input) -> Output;
    fn folded_outputs_mut(&mut self) -> &mut FoldedList<Output>;

    fn transform_all<'a>(&mut self, mut iter: FoldedIter<'a, Input, Storage>)
    where
        Input: 'a,
    {
        while let Some((index, indent, value, children)) = iter.next() {
            // parse current
            self.enter();
            let parse_result = self.transform(indent, value);

            #[cfg(test)]
            assert_eq!(self.folded_outputs_mut().nodes.len(), index);

            let idx = self
                .folded_outputs_mut()
                .append(indent, parse_result, iter.next);
            // parse children
            if let Some(children) = children {
                self.transform_all(children);
            }
            self.exit();
            self.post_exit(idx);
        }
    }
}
