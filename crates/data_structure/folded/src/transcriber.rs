use crate::*;

pub trait Transcriber<Input, InputContainer, Task = ()>
where
    InputContainer: FoldedContainer<Input>,
    Input: ?Sized,
{
    fn enter(&mut self);
    fn exit(&mut self);
    fn post_exit(&mut self, task: Task);
    fn transcribe(&mut self, indent: Indent, input: &Input) -> Option<Task>;

    fn transcribe_all<'a>(&mut self, mut iter: FoldedIter<'a, Input, InputContainer>)
    where
        Input: 'a,
    {
        while let Some(item) = iter.next() {
            self.enter();
            // parse current
            let task = self.transcribe(item.indent, item.value);
            // parse children
            if let Some(children) = item.children {
                self.transcribe_all(children);
            }
            self.exit();
            task.map(|task| self.post_exit(task));
        }
    }
}
