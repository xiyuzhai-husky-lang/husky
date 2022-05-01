mod dev;
mod image28;
mod load;
mod test;
mod val;

use dev_utils::static_dev_src;
use entity_kind::{RoutineKind, TyKind};
use visual_syntax::StaticVisualizer;
use vm::{InputContract, OutputContract, VMError, VMResult};
use xrng::permutation_from_seed;

use super::*;
use crate::*;
use dev::*;
use image28::*;
use load::*;
use test::*;
use val::*;

pub static MNIST_SCOPE_DATA: &EntityStaticDefn = &EntityStaticDefn {
    name: "mnist",
    subscopes: &[
        ("new_binary_dataset", NEW_BINARY_DATASET_SCOPE_DATA),
        ("BinaryImage28", &BINARY_IMAGE_28_TYPE_DEFN),
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
        output_contract: OutputContract::Pure,
        linkage: Linkage {
            call: |_| Ok(StackValue::Boxed(BoxedValue::new(new_binary_dataset()))),
            nargs: 0,
        },
        routine_kind: RoutineKind::Normal,
    },
    dev_src: dev_utils::static_dev_src!(),
};

static BINARY_IMAGE_28_TYPE_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "BinaryImage28",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "datasets::cv::mnist::BinaryImage28",
        generic_placeholders: &[],
        trait_impls: &[
            StaticTraitImplDefn {
                dev_src: static_dev_src!(),
                trai: "Clone",
                member_impls: &[EntityStaticDefn {
                    dev_src: static_dev_src!(),
                    name: "clone",
                    subscopes: &[],
                    variant: EntityStaticDefnVariant::Method {
                        this_contract: InputContract::Pure,
                        input_placeholders: &[],
                        output_ty: "datasets::cv::mnist::BinaryImage28",
                        output_contract: OutputContract::Pure,
                        generic_placeholders: &[],
                        kind: MethodStaticDefnKind::TraitMethodImpl { opt_source: None },
                    },
                }],
            },
            StaticTraitImplDefn {
                dev_src: static_dev_src!(),
                trai: "std::ops::Index<i32>",
                member_impls: &[
                    associated_type_impl!("Output", "b32"),
                    EntityStaticDefn {
                        dev_src: dev_utils::static_dev_src!(),
                        name: "index",
                        subscopes: &[],
                        variant: EntityStaticDefnVariant::Method {
                            this_contract: InputContract::MemberAccess,
                            input_placeholders: &[StaticInputPlaceholder {
                                contract: InputContract::Pure,
                                ty: "i32",
                                name: "todo!()",
                            }],
                            output_ty: "b32",
                            output_contract: OutputContract::MemberAccess,
                            generic_placeholders: &[],
                            kind: MethodStaticDefnKind::TraitMethodImpl {
                                opt_source: Some(LinkageSource::MemberAccess {
                                    ref_access: Linkage {
                                        call: |values| -> VMResult<StackValue> {
                                            let this_value: &BinaryImage28 =
                                                values[0].downcast_ref();
                                            let index_value: usize = values[1]
                                                .as_primitive()
                                                .as_i32()
                                                .try_into()
                                                .expect("todo");
                                            this_value
                                                .get(index_value)
                                                .map(|v| StackValue::Primitive(v.into()))
                                                .ok_or(VMError::Message("todo".into()))
                                        },
                                        nargs: 2,
                                    },
                                    move_access: Linkage {
                                        call: |_| todo!(),
                                        nargs: 2,
                                    },
                                    borrow_mut_access: Linkage {
                                        call: |values| {
                                            let index_value: usize = values[1]
                                                .as_primitive()
                                                .as_i32()
                                                .try_into()
                                                .expect("todo");
                                            let (this_value, owner, _): (&mut BinaryImage28, _, _) =
                                                values[0].downcast_mut_full();
                                            this_value
                                                .get_mut(index_value)
                                                .map(|value| StackValue::MutLocalRef {
                                                    value,
                                                    owner,
                                                    gen: (),
                                                })
                                                .ok_or(VMError::Message("todo".into()))
                                        },
                                        nargs: 2,
                                    },
                                }),
                            },
                        },
                    },
                ],
            },
        ],
        type_members: &[],
        variants: &[],
        kind: TyKind::Array,
        visualizer: StaticVisualizer {
            compiled: BinaryImage28::visualize,
        },
        opt_type_call: Some(&BINARY_IMAGE28_TYPE_CALL_DEFN),
    },
    dev_src: dev_utils::static_dev_src!(),
};

static BINARY_IMAGE28_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryImage28",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Routine {
        generic_placeholders: &[],
        input_placeholders: vec![],
        output_ty: "datasets::cv::mnist::BinaryImage28",
        output_contract: OutputContract::Pure,
        linkage: Linkage {
            call: |_values| Ok(StackValue::Boxed(BoxedValue::new(BinaryImage28::default()))),
            nargs: 0,
        },
        routine_kind: RoutineKind::TypeCall,
    },
    dev_src: static_dev_src!(),
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
