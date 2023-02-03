mod binary_grid28;
mod binary_image28;
mod dev;
mod load;
mod mnist_label;
mod test;
mod val;

pub use binary_grid28::*;
pub use binary_image28::*;
pub use mnist_label::*;

use super::*;
use crate::*;
use dev::*;
use husky_dev_utils::static_dev_src;
use husky_entity_taxonomy::TyKind;
use husky_liason_semantics::OutputModifier;
use husky_vm::*;
use load::*;
use test::*;
use val::*;
use xrng::permutation_from_seed;

pub static MNIST_MOD: &EntityStaticDefn = &EntityStaticDefn {
    name: "mnist",
    items: &[
        &NEW_BINARY_DATASET_SCOPE_DATA,
        &BINARY_IMAGE_28_TYPE_DEFN,
        &BINARY_GRID_28_TYPE_DEFN,
        &MNIST_LABEL_DEFN,
    ],
    variant: EntityStaticDefnVariant::Module,
    dev_src: husky_dev_utils::static_dev_src!(),
};

pub static NEW_BINARY_DATASET_SCOPE_DATA: EntityStaticDefn = EntityStaticDefn {
    name: "new_binary_dataset",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicParameterDecl::None,
        return_ty: "Dataset<cv::datasets::mnist::BinaryImage28, cv::datasets::mnist::MnistLabel>",
        output_liason: OutputModifier::Transfer,
        linkage: transfer_linkage!(
            |_, _| __Register::new_box(new_binary_dataset(), &__DATASET_VTABLE),
            some base new_binary_dataset as fn () -> Dataset<'static>
        )
        .into(),
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};

pub fn new_binary_dataset<'eval>() -> Dataset<'eval> {
    Dataset::new(MnistDataset::new(35016232u64))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MnistDataset {
    images: Arc<Vec<Arc<BinaryImage28>>>,
    labels: Arc<Vec<Label>>,
    permutation: Arc<Vec<u32>>,
}

impl MnistDataset {
    pub fn new(seed: u64) -> Self {
        let (images, labels) = load_mnist();
        Self {
            images,
            labels,
            permutation: Arc::new(permutation_from_seed(60000, seed)),
        }
    }
}

impl Serialize for MnistDataset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("MnistDataset")
    }
}

impl __StaticInfo for MnistDataset {
    type __StaticSelf = Self;

    fn __static_typename() -> Cow<'static, str> {
        todo!()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        std::mem::transmute(self)
    }
}

impl<'eval> __Registrable<'eval> for MnistDataset {
    unsafe fn __to_register(self) -> __Register<'eval> {
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
