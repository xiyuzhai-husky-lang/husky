mod fold_iter;
mod folded_list;
mod local_stack;
mod local_value;
#[cfg(test)]
mod tests;
mod transcriber;
mod transformer;

pub use fold_iter::FoldIter;
pub use folded_list::{FoldIdx, FoldedList, FoldedNode, ItemToFold};
pub use local_stack::LocalStack;
pub use local_value::LocalValue;
pub use transcriber::Transcriber;
pub use transformer::Transformer;

pub trait FoldStorage<Value>
where
    Value: ?Sized,
{
    fn len(&self) -> usize;
    fn indent(&self, index: usize) -> Indent;
    fn next_sibling(&self, index: usize) -> Option<usize>;
    fn value(&self, index: usize) -> &Value;
    fn this(&self) -> &Self;

    fn fold_iter(&self, start: usize) -> FoldIter<Value, Self>
    where
        Self: Sized,
    {
        FoldIter::new(self.this(), self.indent(start), Some(start))
    }
}

pub type Indent = u16;
