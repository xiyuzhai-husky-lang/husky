mod executor;
mod fold_iter;
mod folded_list;
mod local_stack;
mod local_value;
#[cfg(test)]
mod tests;
mod transformer;

use check_utils::should;
pub use executor::Executor;
pub use fold_iter::{FoldIter, FoldIterItem};
pub use folded_list::{FoldIdx, FoldedList, FoldedNode, ItemToFold};
pub use local_stack::LocalStack;
pub use local_value::LocalValue;
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

    fn iter_from(&self, start: usize) -> FoldIter<Value, Self>
    where
        Self: Sized,
    {
        should!(start < self.len());
        FoldIter::new(self.this(), Some(start))
    }

    fn iter(&self) -> FoldIter<Value, Self>
    where
        Self: Sized,
    {
        FoldIter::new(self.this(), if self.len() == 0 { None } else { Some(0) })
    }
}

pub type Indent = u8;
