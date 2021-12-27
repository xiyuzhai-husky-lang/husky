use std::marker::PhantomData;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FoldedList<T> {
    pub(crate) nodes: Vec<FoldedNode<T>>,
}

pub struct FoldedIdx<T> {
    raw: usize,
    phantom: PhantomData<T>,
}

impl<T> std::fmt::Debug for FoldedIdx<T> {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FoldedIdx").field("raw", &self.raw).finish()
    }
}

impl<T> Clone for FoldedIdx<T> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            phantom: self.phantom.clone(),
        }
    }
}

impl<T> Copy for FoldedIdx<T> {}

impl<T> From<usize> for FoldedIdx<T> {
    fn from(raw: usize) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }
}

impl<T> FoldedList<T> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    pub fn nodes(&self) -> &[FoldedNode<T>] {
        &self.nodes
    }

    pub fn append(&mut self, value: T, next_sibling: Option<usize>) -> FoldedIdx<T> {
        let raw = self.nodes.len();
        self.nodes.push(FoldedNode {
            value,
            next_sibling,
        });
        raw.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoldedNode<T> {
    pub(crate) value: T,
    pub(crate) next_sibling: Option<usize>,
}

impl<T> FoldedNode<T> {
    pub fn value(&self) -> &T {
        &self.value
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

impl<T> FoldedStorage<T> for FoldedList<T> {
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
