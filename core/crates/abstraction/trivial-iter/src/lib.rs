use std::marker::PhantomData;

#[derive(Debug)]
pub struct TrivialIter<T> {
    phantom: PhantomData<T>,
}

impl<T> Default for TrivialIter<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T> Iterator for TrivialIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
