use std::marker::PhantomData;

use crate::*;

// abstraction

pub struct LocalInterner<T: Eq>(Vec<T>);
pub struct LocalInternerIdx<T: Eq>(usize, PhantomData<T>);
impl<T: Eq> LocalInternerIdx<T> {
    fn new(raw: usize) -> Self {
        Self(raw, PhantomData)
    }
}

impl<T: Eq> LocalInterner<T> {
    fn intern(&mut self, t: T) -> Option<LocalInternerIdx<T>> {
        let raw = match self.0.iter().position(|s| s == &t) {
            Some(raw) => raw,
            None => {
                let raw = self.0.len();
                self.0.push(t);
                raw
            }
        };
        Some(LocalInternerIdx::new(raw))
    }
}

impl<T: Eq> std::ops::Index<LocalInternerIdx<T>> for LocalInterner<T> {
    type Output = T;

    fn index(&self, index: LocalInternerIdx<T>) -> &Self::Output {
        &self.0[index.0]
    }
}

pub struct LocalTermInterner {
    curries: LocalInterner<LocalTermCurry>,
    // Xin Jiang: add other variants
}

impl LocalTermInterner {
    pub(crate) fn intern_curry(&mut self, curry: LocalTermCurry) -> LocalTermCurryIdx {
        match self.curries.intern(curry) {
            Some(_) => todo!(),
            None => todo!(),
        }
    }
    // Xin Jiang: add other variants
}

pub type LocalTermCurryIdx = LocalInternerIdx<LocalTermCurry>;

impl std::ops::Index<LocalTermCurryIdx> for LocalTermInterner {
    type Output = LocalTermCurry;

    fn index(&self, index: LocalTermCurryIdx) -> &Self::Output {
        &self.curries[index]
    }
}

// Xin Jiang: add other variants
