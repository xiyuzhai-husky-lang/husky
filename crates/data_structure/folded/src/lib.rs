mod iter;
mod list;
mod parser;

pub use iter::FoldedIter;
pub use list::{FoldedList, FoldedNode, ItemToFold};
pub use parser::Parser;

pub trait FoldedStorage<Value, This>
where
    Value: ?Sized,
    This: FoldedStorage<Value, This>,
{
    fn len(&self) -> usize;
    fn next_sibling(&self, index: usize) -> Option<usize>;
    fn value(&self, index: usize) -> &Value;
    fn this(&self) -> &This;

    fn folded_iter(&self, start: usize) -> FoldedIter<Value, This> {
        FoldedIter::new(self.this(), Some(start))
    }
}
