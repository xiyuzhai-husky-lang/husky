pub mod trivial;

use std::marker::PhantomData;
use xrng::XRng;

use crate::*;

pub trait SyntheticDataset<Sample> {
    fn sample(&self, xrng: &mut XRng) -> Sample;
}

pub struct SyntheticSampleIter<'a, Sample, D: SyntheticDataset<Sample>> {
    dataset: &'a D,
    phantom: PhantomData<Sample>,
    xrng: XRng,
}

impl<'a, Sample, D> SyntheticSampleIter<'a, Sample, D>
where
    D: SyntheticDataset<Sample>,
{
    fn new(dataset: &'a D, seed: u64) -> Self {
        SyntheticSampleIter {
            dataset,
            phantom: PhantomData,
            xrng: XRng::new(seed),
        }
    }

    fn new_time_seeded(dataset: &'a D) -> Self {
        SyntheticSampleIter {
            dataset,
            phantom: PhantomData,
            xrng: XRng::new_time_seeded(),
        }
    }
}

impl<'a, D: SyntheticDataset<Sample>, Sample> SampleIterator
    for SyntheticSampleIter<'a, Sample, D>
{
    type Sample = Sample;

    fn next(&mut self) -> Self::Sample {
        self.dataset.sample(&mut self.xrng)
    }
}

impl<'a, Sample: 'a, D: SyntheticDataset<Sample>>
    Dataset<'a, Sample, SyntheticSampleIter<'a, Sample, D>> for D
{
    // train set is reproducible
    fn train_set(&'a self, seed: u64) -> SyntheticSampleIter<'a, Sample, D> {
        SyntheticSampleIter::new(self, seed)
    }

    fn val_set(&'a self) -> SyntheticSampleIter<'a, Sample, D> {
        SyntheticSampleIter::new_time_seeded(self)
    }

    fn test_set(&'a self) -> SyntheticSampleIter<'a, Sample, D> {
        SyntheticSampleIter::new_time_seeded(self)
    }

    fn epoch(&self) -> usize {
        0
    }
}

pub struct SimpleSyntheticDataset<Sample: 'static> {
    gen_sample: fn(&mut XRng) -> Sample,
}

impl<Sample: 'static> SimpleSyntheticDataset<Sample> {
    pub fn new(gen_sample: fn(&mut XRng) -> Sample) -> Self {
        SimpleSyntheticDataset { gen_sample }
    }
}

impl<Sample> SyntheticDataset<Sample> for SimpleSyntheticDataset<Sample> {
    fn sample(&self, xrng: &mut XRng) -> Sample {
        (self.gen_sample)(xrng)
    }
}
