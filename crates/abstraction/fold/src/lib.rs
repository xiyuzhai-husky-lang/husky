mod executor;
mod fold_iter;
mod folded_list;
mod local_stack;
mod local_value;
#[cfg(test)]
mod tests;
mod transformer;

pub use executor::Executor;
pub use fold_iter::{FoldIterItem, FoldableIter};
pub use folded_list::{FoldIdx, FoldableList, FoldedNode, FoldingEnd, ItemToFold};
use husky_check_utils::should;
pub use local_stack::LocalStack;
pub use local_value::LocalValue;
pub use transformer::Transformer;

pub trait FoldableStorage {
    type Value: ?Sized;
    fn len(&self) -> usize;
    fn indent(&self, index: usize) -> Indent;
    fn folding_end(&self, index: usize) -> FoldingEnd;
    fn value(&self, index: usize) -> &Self::Value;
    fn this(&self) -> &Self;

    fn iter_from(&self, start: usize) -> FoldableIter<Self>
    where
        Self: Sized,
    {
        should!(start < self.len());
        FoldableIter::new(self.this(), Some(start))
    }

    fn iter(&self) -> FoldableIter<Self>
    where
        Self: Sized,
    {
        FoldableIter::new(self.this(), if self.len() == 0 { None } else { Some(0) })
    }

    fn next_sibling_idx(&self, idx: usize) -> Option<usize> {
        match self.folding_end(idx) {
            FoldingEnd::Sibling(sibling_idx) => Some(sibling_idx),
            _ => None,
        }
    }
}

pub type Indent = u8;
