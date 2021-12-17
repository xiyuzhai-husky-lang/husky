#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoldedNode<Key> {
    pub(crate) key: Key,
    pub(crate) next_sibling: Option<usize>,
}
