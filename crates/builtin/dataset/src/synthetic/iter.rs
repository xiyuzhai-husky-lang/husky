use xrng::XRng;

use crate::{iter::SampleIterator, *};

use super::*;

pub struct SyntheticSampleIter<'a, D: SyntheticDataset> {
    dataset: &'a D,
    xrng: XRng,
    current: Option<Box<dyn Any>>,
}

impl<'a, D> SyntheticSampleIter<'a, D>
where
    D: SyntheticDataset,
{
    fn new(dataset: &'a D, seed: u64) -> Self {
        SyntheticSampleIter {
            dataset,
            xrng: XRng::new(seed),
            current: None,
        }
    }

    pub(super) fn new_time_seeded(dataset: &'a D) -> Self {
        SyntheticSampleIter {
            dataset,
            xrng: XRng::new_time_seeded(),
            current: None,
        }
    }
}

impl<'a, D: SyntheticDataset> SampleIterator for SyntheticSampleIter<'a, D> {
    fn next(&mut self) -> &dyn Any {
        self.current = Some(self.dataset.sample(self.xrng.randidx()));
        self.current.as_ref().unwrap()
    }
}
