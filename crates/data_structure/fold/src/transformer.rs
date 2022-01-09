use common::*;

use crate::*;

pub trait Transformer<Input, InputContainer, Output = (), Task = ()>
where
    InputContainer: FoldStorage<Input>,
    Input: ?Sized,
{
    fn _enter_block(&mut self);
    fn _exit(&mut self);
    fn transform(
        &mut self,
        indent: Indent,
        input: &Input,
        enter_block: &mut impl FnOnce(&mut Self),
    ) -> Output;
    fn folded_output_mut(&mut self) -> &mut FoldedList<Output>;

    fn transform_all<'a>(&mut self, mut iter: FoldIter<'a, Input, InputContainer>)
    where
        Input: 'a,
    {
        while let Some(item) = iter.next() {
            should_be!(self.folded_output_mut().nodes.len(), item.idx);

            // parse current
            let mut block_entered = false;

            let parse_result = self.transform(item.indent, item.value, &mut |this| {
                block_entered = true;
                this._enter_block();
            });
            self.folded_output_mut()
                .append(item.indent, parse_result, iter.next);
            // parse children
            if !block_entered {
                self._enter_block();
            }
            if let Some(children) = item.children {
                self.transform_all(children);
            }
            self._exit();
        }
    }
}
