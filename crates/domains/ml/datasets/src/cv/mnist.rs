mod dev;
mod image;
mod load;
mod test;
mod val;

use entity_kind::TyKind;
use visual_syntax::StaticVisualizer;
use vm::OutputContract;
use xrng::permutation_from_seed;

use super::*;
use crate::*;
use dev::*;
use image::*;
use load::*;
use test::*;
use val::*;

pub static MNIST_SCOPE_DATA: &StaticEntityDefn = &StaticEntityDefn {
    subscopes: &[
        ("new_binary_dataset", NEW_BINARY_DATASET_SCOPE_DATA),
        ("BinaryImage28", &BINARY_IMAGE_28_TYPE_DEFN),
    ],
    decl: StaticEntityDecl::Module,
};

static NEW_BINARY_DATASET_SCOPE_DATA: &StaticEntityDefn = &StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Func(StaticCallDecl {
        generic_placeholders: &[],
        inputs: vec![],
        output_ty: "Dataset<datasets::cv::mnist::BinaryImage28, i32>",
        output_contract: OutputContract::Pure,
        linkage: Linkage {
            call: |_| Ok(StackValue::Boxed(BoxedValue::new(new_binary_dataset()))),
            nargs: 0,
        },
    }),
};

static BINARY_IMAGE_28_TYPE_DEFN: &StaticEntityDefn = &StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Type(&BINARY_IMAGE28_TYPE_DECL),
};

static BINARY_IMAGE28_TYPE_DECL: StaticTypeDecl = StaticTypeDecl {
    base_route: "datasets::cv::mnist::BinaryImage28",
    generic_placeholders: &[],
    trait_impls: &[StaticTraitImplDecl { route: "Clone" }],
    type_members: &[],
    variants: &[],
    kind: TyKind::Array,
    visualizer: StaticVisualizer {
        compiled: BinaryImage28::visualize,
    },
    opt_type_call: Some(&BINARY_IMAGE28_TYPE_CALL_DECL),
};

static BINARY_IMAGE28_TYPE_CALL_DECL: StaticCallDecl = StaticCallDecl {
    generic_placeholders: &[],
    inputs: vec![],
    output_ty: "datasets::cv::mnist::BinaryImage28",
    output_contract: OutputContract::Pure,
    linkage: Linkage {
        call: |_values| Ok(StackValue::Boxed(BoxedValue::new(BinaryImage28::default()))),
        nargs: 0,
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
