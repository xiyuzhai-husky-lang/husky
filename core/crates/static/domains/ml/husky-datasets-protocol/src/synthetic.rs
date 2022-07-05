mod synthetic_iter;
mod synthetic_loader;

pub use synthetic_iter::*;
pub use synthetic_loader::*;

use super::*;
use husky_trace_protocol::SampleId;
use vm::*;

pub trait SyntheticDataset<'eval>: AnyValueDyn<'eval> + 'eval {
    fn data_generator(&self) -> fn(seed: u64, sample_id: SampleId) -> LabeledData<'eval>;
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

impl<'eval, D: SyntheticDataset<'eval>> DatasetDyn<'eval> for D {
    fn dev_loader(&self) -> DataLoader<'eval> {
        Box::new(SyntheticSampleLoader::new(
            self.dev_len(),
            self.data_generator(),
            self.dev_seed(),
        ))
    }

    fn val_loader(&self) -> DataLoader<'eval> {
        Box::new(SyntheticSampleLoader::new(
            self.val_len(),
            self.data_generator(),
            self.val_seed(),
        ))
    }

    fn test_loader(&self) -> DataLoader<'eval> {
        Box::new(SyntheticSampleLoader::new(
            self.test_len(),
            self.data_generator(),
            self.test_seed(),
        ))
    }

    fn profile_iter(&self) -> DataIter<'eval> {
        todo!()
        // Box::new(SyntheticSampleIter::new(self))
    }
}

#[derive(PartialEq, Eq)]
pub struct SimpleSyntheticDataset<'eval> {
    seed: u64,
    data_generator: fn(seed: u64, sample_id: SampleId) -> LabeledData<'eval>,
}

impl<'eval> std::fmt::Debug for SimpleSyntheticDataset<'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimpleSyntheticDataset")
            .field("gen_sample", &self.data_generator)
            .finish()
    }
}

impl<'eval> Clone for SimpleSyntheticDataset<'eval> {
    fn clone(&self) -> Self {
        Self {
            seed: self.seed,
            data_generator: self.data_generator.clone(),
        }
    }
}

impl<'eval> SimpleSyntheticDataset<'eval> {
    pub fn new(
        seed: u64,
        gen_sample: fn(seed: u64, sample_id: SampleId) -> LabeledData<'eval>,
    ) -> Self {
        SimpleSyntheticDataset {
            seed,
            data_generator: gen_sample,
        }
    }
}

impl<'eval> Serialize for SimpleSyntheticDataset<'eval> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'a> HasStaticTypeInfo for SimpleSyntheticDataset<'a> {
    type StaticSelf = SimpleSyntheticDataset<'static>;

    fn static_type_name() -> Cow<'static, str> {
        todo!()
    }
}

impl<'eval, 'a: 'eval> AnyValue<'eval> for SimpleSyntheticDataset<'a> {
    fn clone_into_box<'temp>(&self) -> Box<dyn AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(self.clone())
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn static_ty() -> EntityRoutePtr {
        todo!()
    }

    fn print_short(&self) -> String {
        todo!()
    }

    fn opt_visualize(
        &'static self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'static dyn AnyValueDyn<'static>,
        ) -> __EvalResult<VisualData>,
    ) -> Option<husky_trace_protocol::VisualData> {
        todo!()
    }
}

impl<'eval> SyntheticDataset<'eval> for SimpleSyntheticDataset<'eval> {
    fn data_generator(&self) -> fn(u64, SampleId) -> LabeledData<'eval> {
        self.data_generator
    }
    fn seed(&self) -> u64 {
        self.seed
    }
}
