mod folded_list;
mod folded_stack;
mod generator;
mod iter;

pub use folded_list::{FoldedIdx, FoldedList, FoldedNode, ItemToFold};
pub use folded_stack::FoldedStack;
pub use generator::Generator;
pub use iter::FoldedIter;

pub trait FoldedStorage<Value>
where
    Value: ?Sized,
{
    fn len(&self) -> usize;
    fn next_sibling(&self, index: usize) -> Option<usize>;
    fn value(&self, index: usize) -> &Value;
    fn this(&self) -> &Self;

    fn folded_iter(&self, start: usize) -> FoldedIter<Value, Self>
    where
        Self: Sized,
    {
        FoldedIter::new(self.this(), Some(start))
    }
}
