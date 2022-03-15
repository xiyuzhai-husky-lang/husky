mod iter;
mod loader;
pub mod trivial;

pub const SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Module,
    subscopes: &[("trivial", trivial::SCOPE_DATA)],
    signature: StaticScopeSignature::Module,
};

use crate::{labeled::LabeledData, *};

use iter::SyntheticSampleIter;
use loader::SyntheticSampleLoader;
use vm::BoxedValue;

pub trait SyntheticDataset: AnyValue + 'static {
    fn data_generator(&self) -> fn(seed: u64, idx: usize) -> LabeledData;
    fn seed(&self) -> u64;
    fn dev_len(&self) -> usize {
        100
    }
    fn dev_seed(&self) -> u64 {
        (self.seed() << 11) & (self.seed() >> 53)
    }
    fn val_len(&self) -> usize {
        100
    }
    fn val_seed(&self) -> u64 {
        (self.seed() << 30) & (self.seed() >> 34)
    }
    fn test_len(&self) -> usize {
        100
    }
    fn test_seed(&self) -> u64 {
        (self.seed() << 41) & (self.seed() >> 23)
    }
}

impl<D: SyntheticDataset> DatasetDyn for D {
    fn dev_loader(&self) -> DataLoader {
        Box::new(SyntheticSampleLoader::new(
            self.dev_len(),
            self.data_generator(),
            self.dev_seed(),
        ))
    }

    fn val_loader(&self) -> DataLoader {
        Box::new(SyntheticSampleLoader::new(
            self.val_len(),
            self.data_generator(),
            self.val_seed(),
        ))
    }

    fn test_loader(&self) -> DataLoader {
        Box::new(SyntheticSampleLoader::new(
            self.test_len(),
            self.data_generator(),
            self.test_seed(),
        ))
    }

    fn profile_iter(&self) -> DataIter {
        Box::new(SyntheticSampleIter::new(self))
    }
}

#[derive(PartialEq, Eq)]
pub struct SimpleSyntheticDataset {
    seed: u64,
    data_generator: fn(seed: u64, idx: usize) -> LabeledData,
}

impl std::fmt::Debug for SimpleSyntheticDataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimpleSyntheticDataset")
            .field("gen_sample", &self.data_generator)
            .finish()
    }
}

impl Clone for SimpleSyntheticDataset {
    fn clone(&self) -> Self {
        Self {
            seed: self.seed,
            data_generator: self.data_generator.clone(),
        }
    }
}

impl SimpleSyntheticDataset {
    pub fn new(seed: u64, gen_sample: fn(seed: u64, idx: usize) -> LabeledData) -> Self {
        SimpleSyntheticDataset {
            seed,
            data_generator: gen_sample,
        }
    }
}

impl AnyValue for SimpleSyntheticDataset {
    fn static_type_id() -> std::any::TypeId {
        todo!()
    }

    fn static_type_name() -> Cow<'static, str> {
        todo!()
    }

    fn boxed_any(&self) -> Box<dyn vm::AnyValueDyn> {
        todo!()
    }

    fn snapshot(&self) -> std::sync::Arc<dyn vm::AnyValueDyn> {
        todo!()
    }
}

impl SyntheticDataset for SimpleSyntheticDataset {
    fn data_generator(&self) -> fn(u64, usize) -> LabeledData {
        self.data_generator
    }
    fn seed(&self) -> u64 {
        self.seed
    }
}
