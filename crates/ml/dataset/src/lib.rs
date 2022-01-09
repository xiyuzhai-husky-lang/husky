mod iter;
pub mod synthetic;

pub use iter::SampleIterator;

pub trait Dataset<'a, Sample: 'a, SampleIter: 'a + SampleIterator<Sample = Sample>> {
    fn epoch(&self) -> usize;
    fn train_set(&'a self, seed: u64) -> SampleIter;
    fn val_set(&'a self) -> SampleIter;
    fn test_set(&'a self) -> SampleIter;
}
