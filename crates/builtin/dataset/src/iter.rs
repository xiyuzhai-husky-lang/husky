use crate::*;

pub trait SampleIterator {
    fn next(&mut self) -> &dyn Any;
}

pub type SampleIter<'a> = Box<dyn SampleIterator + 'a>;
