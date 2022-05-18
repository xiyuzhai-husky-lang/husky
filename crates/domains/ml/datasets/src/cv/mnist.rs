mod binary_grid28;
mod binary_image28;
mod dev;
mod load;
mod test;
mod val;

pub use binary_grid28::*;
pub use binary_image28::*;

use super::*;
use crate::*;
use dev::*;
use dev_utils::static_dev_src;
use entity_kind::{RoutineKind, TyKind};
use load::*;
use test::*;
use val::*;
use visual_syntax::StaticVisualizer;
use vm::*;
use xrng::permutation_from_seed;

pub static MNIST_SCOPE_DATA: &EntityStaticDefn = &EntityStaticDefn {
    name: "mnist",
    subscopes: &[
        ("new_binary_dataset", NEW_BINARY_DATASET_SCOPE_DATA),
        ("BinaryImage28", &BINARY_IMAGE_28_TYPE_DEFN),
        ("BinaryGrid28", &BINARY_GRID_28_TYPE_DEFN),
    ],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};

static NEW_BINARY_DATASET_SCOPE_DATA: &EntityStaticDefn = &EntityStaticDefn {
    name: "new_binary_dataset",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Routine {
        generic_placeholders: &[],
        input_placeholders: vec![],
        output_ty: "Dataset<datasets::cv::mnist::BinaryImage28, i32>",
        output_contract: OutputLiason::Transfer,
        linkage: Linkage {
            call: |_| Ok(StackValue::Boxed(BoxedValue::new(new_binary_dataset()))),
            nargs: 0,
        },
        routine_kind: RoutineKind::Normal,
    },
    dev_src: dev_utils::static_dev_src!(),
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
