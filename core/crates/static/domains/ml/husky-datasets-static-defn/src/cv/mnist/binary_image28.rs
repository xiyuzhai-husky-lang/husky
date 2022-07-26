use super::*;
use husky_entity_route::InternEntityRoute;
use husky_trace_protocol::*;
use husky_visual_syntax::StaticVisualTy;
use std::{any::TypeId, sync::Arc};
use vm::*;

extern "C" {
    pub static __BINARY_IMAGE28_VTABLE: __RegisterVTable;
}

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
                    dev_src: husky_dev_utils::__static_dev_src!(),
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
                        opt_linkage: Some(index_linkage!(
                            BinaryImage28,
                            __BINARY_IMAGE28_VTABLE,
                            u32,
                            __U32_VTABLE,
                            direct
                        )),
                    },
                },
            ],
        }],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Array,
        visual_ty: StaticVisualTy::Image2d,
        opt_type_call: Some(&BINARY_IMAGE28_TYPE_CALL_DEFN),
    },
    dev_src: husky_dev_utils::__static_dev_src!(),
};

pub static BINARY_IMAGE28_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryImage28",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicTemplate::None,
        output_ty: BINARY_IMAGE_28_BASE_ROUTE,
        output_liason: OutputLiason::Transfer,
        linkage: transfer_linkage!(|_, _values| unsafe {
            (__Register::new_box(BinaryImage28::default(), &__BINARY_IMAGE28_VTABLE))
        }, some BinaryImage28::__call__)
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
impl __StaticInfo for BinaryImage28 {
    type __StaticSelf = Self;

    fn __static_type_name() -> Cow<'static, str> {
        todo!()
    }
}

impl __Registrable for BinaryImage28 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        todo!()
    }

    fn __copy__(&self) -> Self {
        panic!()
    }
}
