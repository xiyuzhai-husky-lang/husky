use crate::*;

pub trait BasicInternable: Sized + 'static + std::fmt::Debug {}

impl<T: BasicInternable> Internable for T
where
    T: Eq + std::hash::Hash,
{
    type Borrowed<'a> = &'a T;

    type BorrowedRaw = *const T;

    type Interned = BasicInterned<Self>;

    fn borrow<'a>(&'a self) -> Self::Borrowed<'a> {
        todo!()
    }

    fn new_itr() -> Interner<Self> {
        todo!()
    }

    fn try_direct(&self) -> Option<Self::Interned> {
        None
    }

    fn itd_to_borrowed(itd: Self::Interned) -> Self::Borrowed<'static> {
        itd.0
    }

    fn to_borrowed<'a>(&'a self) -> Self::Borrowed<'a> {
        todo!()
    }

    fn new_itd(&'static self, id: usize) -> Self::Interned {
        todo!()
    }
}

#[derive(Debug)]
pub struct BasicInterned<T: BasicInternable>(&'static T);

impl<T: BasicInternable> Clone for BasicInterned<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: BasicInternable> Copy for BasicInterned<T> {}
