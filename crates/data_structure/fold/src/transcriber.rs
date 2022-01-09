use crate::*;

pub trait Transcriber<Input, InputContainer>
where
    InputContainer: FoldStorage<Input>,
    Input: ?Sized,
{
    fn enter_fold(&mut self);
    fn enter_block(&mut self);
    fn exit(&mut self);
    fn transcribe(&mut self, indent: Indent, input: &Input);

    fn transcribe_all<'a>(&mut self, mut iter: FoldIter<'a, Input, InputContainer>)
    where
        Input: 'a,
    {
        while let Some(item) = iter.next() {
            // parse current
            self.enter_fold();
            // parse children
            self.enter_block();
            if let Some(children) = item.children {
                self.transcribe_all(children);
            }
            self.exit();
        }
    }
}
