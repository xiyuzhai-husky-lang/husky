use common::*;

use crate::*;

pub trait Transformer<Input, InputContainer, Output, Task = ()>
where
    InputContainer: FoldedContainer<Input>,
    Input: ?Sized,
{
    fn enter(&mut self);
    fn exit(&mut self);
    fn designate_task(&self, output: &Output) -> Option<Task>;
    fn post_exit(&mut self, task: Task);
    fn transform(&mut self, indent: Indent, input: &Input) -> Output;
    fn folded_outputs_mut(&mut self) -> &mut FoldedList<Output>;

    fn transform_all<'a>(&mut self, mut iter: FoldedIter<'a, Input, InputContainer>)
    where
        Input: 'a,
    {
        while let Some(item) = iter.next() {
            should_be!(self.folded_outputs_mut().nodes.len(), item.idx);

            // parse current
            self.enter();
            let parse_result = self.transform(item.indent, item.value);
            let task = self.designate_task(&parse_result);
            self.folded_outputs_mut()
                .append(item.indent, parse_result, iter.next);
            // parse children
            if let Some(children) = item.children {
                self.transform_all(children);
            }
            self.exit();
            task.map(|task| self.post_exit(task));
        }
    }
}
