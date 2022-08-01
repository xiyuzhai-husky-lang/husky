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
use entity_kind::{RoutineKind, TyKind};
use husky_dev_utils::static_dev_src;
use husky_entity_route::EntityRoutePtr;
use husky_liason_semantics::OutputLiason;
use load::*;
use test::*;
use val::*;
use vm::*;
use xrng::permutation_from_seed;

pub static MNIST_SCOPE_DATA: &EntityStaticDefn = &EntityStaticDefn {
    name: "mnist",
    items: &[
        NEW_BINARY_DATASET_SCOPE_DATA,
        &BINARY_IMAGE_28_TYPE_DEFN,
        &BINARY_GRID_28_TYPE_DEFN,
    ],
    variant: EntityStaticDefnVariant::Module,
    dev_src: husky_dev_utils::static_dev_src!(),
};

pub static NEW_BINARY_DATASET_SCOPE_DATA: &EntityStaticDefn = &EntityStaticDefn {
    name: "new_binary_dataset",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicTemplate::None,
        output_ty: "Dataset<domains::ml::datasets::cv::mnist::BinaryImage28, i32>",
        output_liason: OutputLiason::Transfer,
        linkage: transfer_linkage!(
            |_, _|  unsafe {__Register::new_box(new_binary_dataset(), &__DATASET_VTABLE)} ,
            some new_binary_dataset
        )
        .into(),
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};

pub fn new_binary_dataset<'eval>() -> Dataset<'eval> {
    Dataset::new(MnistDataset::new(35016232u64))
}
