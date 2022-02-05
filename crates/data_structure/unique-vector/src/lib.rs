use std::{borrow::Borrow, iter::Iterator, ops::Deref};

use common::should_eq;

pub struct UniqVec<T>
where
    T: PartialEq + Eq,
{
    v: Vec<T>,
}

impl<T> Default for UniqVec<T>
where
    T: PartialEq + Eq,
{
    fn default() -> Self {
        Self { v: vec![] }
    }
}

impl<T, Iter: Iterator<Item = T>> From<Iter> for UniqVec<T>
where
    T: PartialEq + Eq,
{
    fn from(iter: Iter) -> Self {
        let mut vec = vec![];
        for item in iter {
            if !vec.contains(&item) {
                vec.push(item)
            }
        }
        Self { v: vec }
    }
}

impl<T> UniqVec<T>
where
    T: PartialEq + Eq,
{
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.v.iter()
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }
}

impl<T> IntoIterator for UniqVec<T>
where
    T: PartialEq + Eq,
{
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
    }
}

impl<T> Deref for UniqVec<T>
where
    T: PartialEq + Eq,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

#[test]
fn test_uniq() {
    let a: UniqVec<i32> = [1, 2, 3, 1, 2, 3, 1, 3].into_iter().into();
    should_eq!(a.len(), 3);
}
