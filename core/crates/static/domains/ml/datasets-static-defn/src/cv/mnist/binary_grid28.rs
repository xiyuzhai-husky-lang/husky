use super::*;
use husky_tracer_protocol::*;
use liason::{MemberLiason, ParameterLiason};
use std::any::TypeId;
use visual_syntax::{StaticVisualTy, StaticVisualizerVariant};
use vm::*;

pub static BINARY_GRID_28_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryGrid28",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "domains::ml::datasets::cv::mnist::BinaryGrid28",
        generic_parameters: &[],
        static_trait_impls: &[StaticTraitImplDefn {
            dev_src: static_dev_src!(),
            trai: "std::ops::Index<i32>",
            member_impls: &[
                associated_type_impl!("Output", "b32"),
                EntityStaticDefn {
                    dev_src: dev_utils::static_dev_src!(),
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
                        generic_parameters: &[],
                        kind: MethodStaticDefnVariant::TraitMethodImpl {
                            opt_source: Some(LinkageSource::MemberAccess {
                                copy_access: routine_linkage!(
                                    |values| -> VMRuntimeResult<TempValue> {
                                        let this_value: &BinaryGrid28 = values[0].downcast_ref();
                                        let index_value: usize = values[1]
                                            .take_copyable()
                                            .take_i32()
                                            .try_into()
                                            .expect("todo");
                                        this_value
                                            .get(index_value)
                                            .map(|v| TempValue::Copyable(v.into()))
                                            .ok_or(VMRuntimeError {
                                                message: "todo".into(),
                                            })
                                    },
                                    2
                                ),
                                eval_ref_access: routine_linkage!(
                                    |values| -> VMRuntimeResult<TempValue> { todo!() },
                                    2
                                ),
                                temp_ref_access: routine_linkage!(
                                    |values| -> VMRuntimeResult<TempValue> { todo!() },
                                    2
                                ),
                                move_access: routine_linkage!(|_| todo!(), 2),
                                temp_mut_access: routine_linkage!(
                                    |values| {
                                        let index_value: usize = values[1]
                                            .take_copyable()
                                            .take_i32()
                                            .try_into()
                                            .expect("todo");
                                        let (this_value, owner, _): (&mut BinaryGrid28, _, _) =
                                            values[0].downcast_mut_full();
                                        this_value
                                            .get_mut(index_value)
                                            .map(|value| TempValue::TempRefMutEval {
                                                value,
                                                owner,
                                                gen: (),
                                            })
                                            .ok_or(VMRuntimeError {
                                                message: "todo".into(),
                                            })
                                    },
                                    2
                                ),
                            }),
                        },
                    },
                },
            ],
        }],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Array,
        visualizer: &StaticVisualizer {
            variant: StaticVisualizerVariant::Compiled {
                call: BinaryGrid28::visualize,
            },
            ty: StaticVisualTy::Shape2d,
        },
        opt_type_call: Some(&BINARY_GRID28_TYPE_CALL_DEFN),
    },
    dev_src: dev_utils::static_dev_src!(),
};

pub static BINARY_GRID28_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryGrid28",
    items: &[],
    variant: EntityStaticDefnVariant::Routine {
        generic_parameters: &[],
        parameters: &[],
        output_ty: "domains::ml::datasets::cv::mnist::BinaryGrid28",
        output_liason: OutputLiason::Transfer,
        linkage: routine_linkage!(
            |_values| {
                Ok(TempValue::OwnedEval(OwnedValue::new(
                    BinaryGrid28::default(),
                )))
            },
            0
        ),
        routine_kind: RoutineKind::TypeCall,
    },
    dev_src: static_dev_src!(),
};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct BinaryGrid28 {
    padded_rows: [u32; 31],
}

impl std::fmt::Debug for BinaryGrid28 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "BinaryGrid {{ padded_rows: [{:?}] }}",
            self.padded_rows
        ))
    }
}

impl BinaryGrid28 {
    pub fn visualize<'temp, 'eval>(value: &(dyn AnyValueDyn<'eval> + 'temp)) -> VisualData {
        let value: &BinaryGrid28 = value.downcast_ref();
        VisualData::BinaryGrid28 {
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

impl Serialize for BinaryGrid28 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'eval> AnyValue<'eval> for BinaryGrid28 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    // fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
    //     Arc::new(self.clone())
    // }

    fn print_short(&self) -> String {
        "BinaryGrid28 { ... }".into()
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
}
