use super::*;
use std::{any::TypeId, sync::Arc};
use visual_syntax::VisualProps;
use vm::{AnyValue, AnyValueDyn, StaticTypeId};

pub static BINARY_GRID_28_TYPE_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "BinaryGrid28",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "datasets::cv::mnist::BinaryGrid28",
        generic_parameters: &[],
        static_trait_impls: &[StaticTraitImplDefn {
            dev_src: static_dev_src!(),
            trai: "std::ops::Index<i32>",
            member_impls: &[
                associated_type_impl!("Output", "b32"),
                EntityStaticDefn {
                    dev_src: dev_utils::static_dev_src!(),
                    name: "index",
                    subscopes: &[],
                    variant: EntityStaticDefnVariant::Method {
                        this_contract: ParameterLiason::MemberAccess,
                        parameters: &[StaticParameter {
                            contract: ParameterLiason::Pure,
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
                                copy_access: linkage!(
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
                                ref_access: linkage!(
                                    |values| -> VMRuntimeResult<TempValue> { todo!() },
                                    2
                                ),
                                move_access: linkage!(|_| todo!(), 2),
                                ref_mut_access: linkage!(
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
                                            .map(|value| TempValue::CopyableOrFullyOwnedMut {
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
        visualizer: StaticVisualizer::Compiled(BinaryGrid28::visualize),
        opt_type_call: Some(&BINARY_GRID28_TYPE_CALL_DEFN),
    },
    dev_src: dev_utils::static_dev_src!(),
};

pub static BINARY_GRID28_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryGrid28",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Routine {
        generic_parameters: &[],
        parameters: vec![],
        output_ty: "datasets::cv::mnist::BinaryGrid28",
        output_liason: OutputLiason::Transfer,
        linkage: linkage!(
            |_values| {
                Ok(TempValue::EvalOwned(OwnedValue::new(
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
    pub fn visualize<'eval>(value: &(dyn AnyValueDyn<'eval> + 'eval)) -> VisualProps {
        let value: &BinaryGrid28 = value.downcast_ref();
        VisualProps::BinaryGrid28 {
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
}
