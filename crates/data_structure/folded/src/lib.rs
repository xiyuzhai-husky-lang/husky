mod iter;
mod list;
mod node;
mod parser;

pub use iter::FoldedIter;
pub use list::{FoldedList, ItemToFold};
pub use node::FoldedNode;
pub use parser::Parser;

pub trait Folded<Key, Value, This>
where
    This: Folded<Key, Value, This>,
    Value: ?Sized,
{
    fn nodes(&self) -> &[FoldedNode<Key>];
    fn value(&self, key: &Key) -> &Value;
    fn this(&self) -> &This;

    fn folded_iter(&self, start: usize) -> FoldedIter<Key, Value, This> {
        FoldedIter::new(self.this(), Some(start))
    }
}
