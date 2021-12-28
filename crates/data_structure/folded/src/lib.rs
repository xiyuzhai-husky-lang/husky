mod folded_iter;
mod folded_list;
mod folded_stack;
#[cfg(test)]
mod tests;
mod transformer;

pub use folded_iter::FoldedIter;
pub use folded_list::{FoldedIdx, FoldedList, FoldedNode, ItemToFold};
pub use folded_stack::FoldedStack;
pub use transformer::Transformer;

pub trait FoldedContainer<Value>
where
    Value: ?Sized,
{
    fn len(&self) -> usize;
    fn indent(&self, index: usize) -> Indent;
    fn next_sibling(&self, index: usize) -> Option<usize>;
    fn value(&self, index: usize) -> &Value;
    fn this(&self) -> &Self;

    fn folded_iter(&self, start: usize) -> FoldedIter<Value, Self>
    where
        Self: Sized,
    {
        FoldedIter::new(self.this(), self.indent(start), Some(start))
    }
}

pub type Indent = u16;
