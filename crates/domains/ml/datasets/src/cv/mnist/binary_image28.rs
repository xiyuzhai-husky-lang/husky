use super::*;
use std::{any::TypeId, sync::Arc};
use visual_syntax::VisualProps;
use vm::{AnyValue, AnyValueDyn, StaticTypeId};

pub static BINARY_IMAGE_28_TYPE_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "BinaryImage28",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "datasets::cv::mnist::BinaryImage28",
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
                                copy_access: linkage!(
                                    |values| -> VMRuntimeResult<TempValue> {
                                        let this_value: &BinaryImage28 = values[0].downcast_ref();
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
                                eval_ref_access: linkage!(
                                    |values| -> VMRuntimeResult<TempValue> { todo!() },
                                    2
                                ),
                                temp_ref_access: linkage!(
                                    |values| -> VMRuntimeResult<TempValue> { todo!() },
                                    2
                                ),
                                move_access: linkage!(|_| todo!(), 2),
                                temp_mut_access: linkage!(
                                    |values| {
                                        let index_value: usize = values[1]
                                            .take_copyable()
                                            .take_i32()
                                            .try_into()
                                            .expect("todo");
                                        let (this_value, owner, _): (&mut BinaryImage28, _, _) =
                                            values[0].downcast_mut_full();
                                        this_value
                                            .get_mut(index_value)
                                            .map(|value| TempValue::CopyableOrTempMutEval {
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
        visualizer: StaticVisualizer::Compiled(BinaryImage28::visualize),
        opt_type_call: Some(&BINARY_IMAGE28_TYPE_CALL_DEFN),
    },
    dev_src: dev_utils::static_dev_src!(),
};

pub static BINARY_IMAGE28_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryImage28",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Routine {
        generic_parameters: &[],
        parameters: vec![],
        output_ty: "datasets::cv::mnist::BinaryImage28",
        output_liason: OutputLiason::Transfer,
        linkage: linkage!(
            |_values| {
                Ok(TempValue::OwnedEval(OwnedValue::new(
                    BinaryImage28::default(),
                )))
            },
            0
        ),
        paradigm: RoutineKind::TypeCall,
    },
    dev_src: static_dev_src!(),
};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct BinaryImage28 {
    padded_rows: [u32; 30],
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

    pub fn visualize<'eval>(value: &(dyn AnyValueDyn<'eval> + 'eval)) -> VisualProps {
        let value: &BinaryImage28 = value.downcast_ref();
        VisualProps::BinaryImage28 {
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

impl<'eval> AnyValue<'eval> for BinaryImage28 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "BinaryImage28".into()
    }

    fn print_short(&self) -> String {
        "BinaryImage28 { ... }".into()
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }
}
