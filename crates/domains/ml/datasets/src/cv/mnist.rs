mod dev;
mod image;
mod load;
mod test;
mod val;

use entity_syntax::RawTyKind;
use visual_syntax::BuiltinVisualizer;
use xrng::permutation_from_seed;

use super::*;
use crate::*;
use dev::*;
use image::*;
use load::*;
use test::*;
use val::*;

pub const MNIST_SCOPE_DATA: &BuiltinEntityData = &BuiltinEntityData {
    subscopes: &[
        ("new_binary_dataset", NEW_BINARY_DATASET_SCOPE_DATA),
        ("BinaryImage28", BINARY_IMAGE_28_SCOPE_DATA),
    ],
    signature: BuiltinScopeSignature::Module,
};

const NEW_BINARY_DATASET_SCOPE_DATA: &BuiltinEntityData = &BuiltinEntityData {
    subscopes: &[],
    signature: BuiltinScopeSignature::Func(StaticFuncSignature {
        inputs: vec![],
        output: "Dataset<datasets::cv::mnist::BinaryImage28, i32>",
        compiled: Some(Compiled {
            call: |_| Ok(StackValue::Boxed(BoxedValue::new(new_binary_dataset()))),
        }),
    }),
};

const BINARY_IMAGE_28_SCOPE_DATA: &BuiltinEntityData = &BuiltinEntityData {
    subscopes: &[],
    signature: BuiltinScopeSignature::Ty {
        visualizer: BuiltinVisualizer {
            compiled: BinaryImage28::visualize,
        },
        raw_ty_kind: RawTyKind::Other,
    },
};

pub fn new_binary_dataset<'eval>() -> Dataset<'eval> {
    Dataset::new(MnistDataset::new(35016232u64))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MnistDataset {
    images: Arc<Vec<Arc<BinaryImage28>>>,
    labels: Arc<Vec<u8>>,
    permutation: Arc<Vec<u32>>,
}

impl MnistDataset {
    pub fn new(seed: u64) -> Self {
        let (images, labels) = load();
        Self {
            images,
            labels,
            permutation: Arc::new(permutation_from_seed(60000, seed)),
        }
    }
}

impl<'eval> AnyValue<'eval> for MnistDataset {
    fn static_type_id() -> StaticTypeId {
        todo!()
    }

    fn static_type_name() -> Cow<'static, str> {
        todo!()
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        todo!()
    }
}

impl<'eval> DatasetDyn<'eval> for MnistDataset {
    fn dev_loader(&self) -> DataLoader<'eval> {
        Box::new(MnistDevLoader::new(
            self.images.clone(),
            self.labels.clone(),
            self.permutation.clone(),
        ))
    }

    fn val_loader(&self) -> DataLoader<'eval> {
        Box::new(MnistValLoader::new(self.permutation.clone()))
    }

    fn test_loader(&self) -> DataLoader<'eval> {
        Box::new(MnistTestLoader::new(self.permutation.clone()))
    }

    fn profile_iter(&self) -> DataIter<'eval> {
        todo!()
    }
}
