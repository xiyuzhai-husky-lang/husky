use crate::*;

pub trait Executor<Input, InputContainer>
where
    InputContainer: FoldStorage<Input>,
    Input: ?Sized,
{
    fn _enter_block(&mut self);
    fn _exit_block(&mut self);
    fn execute(&mut self, indent: Indent, input: &Input, enter_block: impl FnOnce(&mut Self));

    fn execute_all<'a>(&mut self, mut iter: FoldIter<'a, Input, InputContainer>)
    where
        Input: 'a,
    {
        while let Some(item) = iter.next() {
            let mut block_entered = false;

            // parse current
            self.execute(item.indent, item.value, |this| {
                block_entered = true;
                this._enter_block();
            });

            if !block_entered {
                self._enter_block();
            }

            // parse children
            if let Some(children) = item.opt_children {
                self.execute_all(children);
            }
            self._exit_block();
        }
    }
}
