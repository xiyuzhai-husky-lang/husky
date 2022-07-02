use super::*;
use husky_entity_route::{lazy_entity_route, AllocateUniqueScope};
use husky_eval_time::{compile_time, parse_entity_route};
use husky_trace_protocol::*;
use husky_visual_syntax::{StaticVisualTy, StaticVisualizerVariant};
use std::{any::TypeId, sync::Arc};
use vm::*;

pub static BINARY_IMAGE_28_BASE_ROUTE: &'static str =
    "domains::ml::datasets::cv::mnist::BinaryImage28";

pub static BINARY_IMAGE_28_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryImage28",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: BINARY_IMAGE_28_BASE_ROUTE,
        spatial_parameters: &[],
        static_trait_impls: &[StaticTraitImplDefn {
            dev_src: __static_dev_src!(),
            trai: "std::ops::Index<i32>",
            member_impls: &[
                associated_type_impl!("Output", "b32"),
                EntityStaticDefn {
                    dev_src: dev_utils::__static_dev_src!(),
                    name: "index",
                    items: &[],
                    variant: EntityStaticDefnVariant::Method {
                        this_liason: ParameterLiason::MemberAccess,
                        parameters: &[StaticParameter {
                            liason: ParameterLiason::Pure,
                            ty: "i32",
                            name: "todo!()",
                        }],
                        output_ty: "b32",
                        output_liason: OutputLiason::MemberAccess {
                            member_liason: MemberLiason::Mutable,
                        },
                        spatial_parameters: &[],
                        method_static_defn_kind: MethodStaticDefnKind::TraitMethodImpl,
                        opt_linkage: Some(__Linkage::Member(&__MemberLinkage {
                            copy_access: index_copy_fp!(BinaryImage28),
                            eval_ref_access: __SpecificRoutineFp(
                                |values| -> __EvalResult<__TempValue> { todo!() },
                            ),
                            temp_ref_access: __SpecificRoutineFp(
                                |values| -> __EvalResult<__TempValue> { todo!() },
                            ),
                            move_access: __SpecificRoutineFp(|_| todo!()),
                            temp_mut_access: __SpecificRoutineFp(|values| {
                                let index_value: usize = values[1]
                                    .take_copyable()
                                    .take_i32()
                                    .try_into()
                                    .expect("todo");
                                let (this_value, owner, _): (&mut BinaryImage28, _, _) =
                                    values[0].downcast_mut_full();
                                this_value
                                    .get_mut(index_value)
                                    .map(|value| __TempValue::TempRefMutEval {
                                        value,
                                        owner,
                                        gen: (),
                                    })
                                    .ok_or(EvalError::Normal {
                                        message: "todo".into(),
                                    })
                            }),
                            nargs: 2,
                            dev_src: __static_dev_src!(),
                        })),
                    },
                },
            ],
        }],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Array,
        visualizer: &StaticVisualizer {
            ty: StaticVisualTy::Image2d,
            variant: StaticVisualizerVariant::Compiled {
                call: BinaryImage28::visualize,
            },
        },
        opt_type_call: Some(&BINARY_IMAGE28_TYPE_CALL_DEFN),
    },
    dev_src: dev_utils::__static_dev_src!(),
};

pub static BINARY_IMAGE28_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryImage28",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        output_ty: "domains::ml::datasets::cv::mnist::BinaryImage28",
        output_liason: OutputLiason::Transfer,
        linkage: specific_transfer_linkage!(
            |_values| {
                Ok(__TempValue::OwnedEval(OwnedValue::new(
                    BinaryImage28::default(),
                )))
            },
            0
        )
        .into(),
    },
    dev_src: __static_dev_src!(),
};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct BinaryImage28 {
    padded_rows: [u32; 30],
}

impl std::ops::Index<usize> for BinaryImage28 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.padded_rows[index]
    }
}

impl std::ops::IndexMut<usize> for BinaryImage28 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.padded_rows[index]
    }
}

impl std::fmt::Debug for BinaryImage28 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "BinaryImage {{ padded_rows: [{:?}] }}",
            self.padded_rows
        ))
    }
}

impl BinaryImage28 {
    pub fn __call__() -> Self {
        Self {
            padded_rows: Default::default(),
        }
    }

    pub fn read(content: &[u8]) -> Self {
        assert_eq!(content.len(), 28 * 4);
        let mut padded_rows = [0; 30];
        for i in 0..28 {
            let mut row = 0u32;
            for k in 0..4 {
                row |= (content[i * 4 + k] as u32) << (3 - k) * 8;
            }
            padded_rows[i + 1] = row;
        }
        Self { padded_rows }
    }

    pub fn visualize<'temp, 'eval>(value: &(dyn AnyValueDyn<'eval> + 'temp)) -> VisualData {
        let value: &BinaryImage28 = value.downcast_ref();
        VisualData::BinaryImage28 {
            padded_rows: value.padded_rows.clone(),
        }
    }

    pub(crate) fn get(&self, index: usize) -> Option<u32> {
        self.padded_rows.get(index).map(|x| *x)
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut u32> {
        self.padded_rows.get_mut(index)
    }
}

impl Serialize for BinaryImage28 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}
impl HasStaticTypeInfo for BinaryImage28 {
    type StaticSelf = Self;

    fn static_type_name() -> Cow<'static, str> {
        todo!()
    }
}

impl<'eval> AnyValue<'eval> for BinaryImage28 {
    fn print_short(&self) -> String {
        "BinaryImage28 { ... }".into()
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

    fn ty(&self) -> EntityRoutePtr {
        lazy_entity_route!(BINARY_IMAGE_28_BASE_ROUTE)
    }
}
