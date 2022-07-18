mod binary_grid28;
mod binary_image28;
mod dev;
mod load;
mod test;
mod val;

pub use binary_grid28::*;
pub use binary_image28::*;
use husky_entity_route::EntityRoutePtr;
use husky_liason_semantics::OutputLiason;

use super::*;
use crate::*;
use dev::*;
use dev_utils::__static_dev_src;
use entity_kind::{RoutineKind, TyKind};
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
    dev_src: dev_utils::__static_dev_src!(),
};

pub static NEW_BINARY_DATASET_SCOPE_DATA: &EntityStaticDefn = &EntityStaticDefn {
    name: "new_binary_dataset",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicTemplateDefn::None,
        output_ty: "Dataset<domains::ml::datasets::cv::mnist::BinaryImage28, i32>",
        output_liason: OutputLiason::Transfer,
        linkage: specific_transfer_linkage!(
            |_, _| __TempValue::OwnedEval(__OwnedValue::new(new_binary_dataset())),
            0
        )
        .into(),
    },
    dev_src: dev_utils::__static_dev_src!(),
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

impl __HasStaticTypeInfo for MnistDataset {
    type __StaticSelf = Self;

    fn __static_type_name() -> Cow<'static, str> {
        todo!()
    }
}

impl<'eval> __AnyValue<'eval> for MnistDataset {
    fn __to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        todo!()
    }

    fn __static_ty() -> EntityRoutePtr {
        todo!()
    }

    fn __print_short(&self) -> String {
        todo!()
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<husky_trace_protocol::VisualData>> {
        todo!()
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        todo!()
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
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
