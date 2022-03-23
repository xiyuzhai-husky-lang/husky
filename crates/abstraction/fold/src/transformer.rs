use crate::*;

pub trait Transformer<Input, InputContainer, Output>
where
    InputContainer: FoldStorage<Input>,
    Input: ?Sized,
{
    fn _enter_block(&mut self);
    fn _exit_block(&mut self);
    fn transform(
        &mut self,
        indent: Indent,
        input: &Input,
        enter_block: impl FnOnce(&mut Self),
    ) -> Output;
    fn folded_output_mut(&mut self) -> &mut FoldedList<Output>;

    fn transform_all<'a>(&mut self, mut iter: FoldIter<'a, Input, InputContainer>)
    where
        Input: 'a,
    {
        while let Some(item) = iter.next() {
            #[cfg(test)]
            {
                use check_utils::*;
                should_eq!(self.folded_output_mut().nodes.len(), item.idx);
            }

            let mut block_entered = false;

            // parse current
            let parse_result = self.transform(item.indent, item.value, |this| {
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
            self._exit_block();
        }
    }
}
