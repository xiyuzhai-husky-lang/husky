use crate::*;

pub trait DataIterator {
    fn next(&mut self) -> LabeledData;
}

pub type DataIter<'a> = Box<dyn DataIterator + 'a>;
