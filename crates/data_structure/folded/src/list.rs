use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FoldedList<Key> {
    pub(crate) nodes: Vec<FoldedNode<Key>>,
}

impl<Key> FoldedList<Key> {
    pub fn nodes(&self) -> &[FoldedNode<Key>] {
        &self.nodes
    }
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
                key: items[i].key(),
                next_sibling,
            })
        }

        Self { nodes }
    }
}

pub trait ItemToFold<Key> {
    fn key(&self) -> Key;
    fn indent(&self) -> u16;
}
