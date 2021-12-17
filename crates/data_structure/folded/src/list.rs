use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FoldedList<T> {
    pub(crate) nodes: Vec<FoldedNode<T>>,
}

impl<T> FoldedList<T> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    pub fn nodes(&self) -> &[FoldedNode<T>] {
        &self.nodes
    }

    pub fn append(&mut self, value: T, next_sibling: Option<usize>) {
        self.nodes.push(FoldedNode {
            value,
            next_sibling,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoldedNode<T> {
    pub(crate) value: T,
    pub(crate) next_sibling: Option<usize>,
}

impl<Item, T> From<Vec<Item>> for FoldedList<T>
where
    Item: ItemToFold<T>,
{
    fn from(items: Vec<Item>) -> Self {
        let mut nodes = Vec::new();
        nodes.reserve(items.len());

        for i in 0..items.len() {
            let mut j = i + 1;
            let next_sibling = loop {
                if j < items.len() {
                    if items[j].indent() <= items[i].indent() {
                        break Some(j);
                    }
                    j += 1;
                } else {
                    break None;
                }
            };
            nodes.push(FoldedNode {
                value: items[i].value(),
                next_sibling,
            })
        }

        Self { nodes }
    }
}

pub trait ItemToFold<Key> {
    fn value(&self) -> Key;
    fn indent(&self) -> u16;
}

impl<T> FoldedStorage<T, FoldedList<T>> for FoldedList<T> {
    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn next_sibling(&self, index: usize) -> Option<usize> {
        self.nodes[index].next_sibling
    }

    fn value(&self, index: usize) -> &T {
        &self.nodes[index].value
    }

    fn this(&self) -> &FoldedList<T> {
        self
    }
}
