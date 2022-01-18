mod iter;
mod loader;
pub mod trivial;

pub const SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Module,
    subscopes: &[("trivial", trivial::SCOPE_DATA)],
    call_signature: None,
    compiled: None,
};

use crate::*;

use iter::SyntheticSampleIter;
use loader::SyntheticSampleLoader;

pub trait SyntheticDataset {
    fn sample(&self, idx: usize) -> Box<dyn Any>;
}

impl<D: SyntheticDataset> Dataset for D {
    fn dev_loader(&self) -> SampleLoader {
        Box::new(SyntheticSampleLoader::new(self))
    }

    fn val_iter(&self) -> SampleIter {
        Box::new(SyntheticSampleIter::new_time_seeded(self))
    }

    fn test_iter(&self) -> SampleIter {
        Box::new(SyntheticSampleIter::new_time_seeded(self))
    }
}

#[derive(Debug)]
pub struct SimpleSyntheticDataset<Sample: 'static> {
    gen_sample: fn(idx: usize) -> Sample,
}

impl<Sample: 'static> Clone for SimpleSyntheticDataset<Sample> {
    fn clone(&self) -> Self {
        Self {
            gen_sample: self.gen_sample.clone(),
        }
    }
}

impl<Sample> virtual_stack::HasRef for SimpleSyntheticDataset<Sample> {
    fn has_ref(&self) -> bool {
        false
    }
}

impl<Sample: 'static> SimpleSyntheticDataset<Sample> {
    pub fn new(gen_sample: fn(idx: usize) -> Sample) -> Self {
        SimpleSyntheticDataset { gen_sample }
    }
}

impl<Sample> SyntheticDataset for SimpleSyntheticDataset<Sample> {
    fn sample(&self, idx: usize) -> Box<dyn Any> {
        Box::new((self.gen_sample)(idx))
    }
}
