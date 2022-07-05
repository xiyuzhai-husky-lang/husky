use husky_trace_protocol::*;
use husky_visual_syntax::StaticVisualTy;

use super::*;

pub trait __B32X {
    fn ctz(self) -> i32;
    fn clz(self) -> i32;
    fn last_bits(self, n: i32) -> u32;
}

impl __B32X for u32 {
    fn ctz(self) -> i32 {
        self.trailing_zeros() as i32
    }

    fn clz(self) -> i32 {
        self.leading_zeros() as i32
    }
    #[inline(always)]
    fn last_bits(self, n: i32) -> u32 {
        self & !(u32::MAX << n)
    }
}

pub static B32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "b32",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "b32",
        spatial_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[&B32_LEADING_ZEROS, &B32_TRAILING_ZEROS, &B32_LAST_BITS],
        variants: &[],
        kind: TyKind::Primitive,
        visual_ty: StaticVisualTy::B32,
        opt_type_call: None,
    },
    dev_src: __static_dev_src!(),
};

pub static B32_LEADING_ZEROS: EntityStaticDefn = EntityStaticDefn {
    name: "leading_zeros",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(
            |values| {
                __TempValue::Copyable(
                    (values[0].take_copyable().take_b32().leading_zeros() as i32).into(),
                )
            },
            1
        )),
    },
    dev_src: __static_dev_src!(),
};

pub static B32_TRAILING_ZEROS: EntityStaticDefn = EntityStaticDefn {
    name: "ctz",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(
            |values| { __TempValue::Copyable((values[0].take_copyable().take_b32().ctz()).into()) },
            1
        )),
    },
    dev_src: __static_dev_src!(),
};

pub static B32_LAST_BITS: EntityStaticDefn = EntityStaticDefn {
    name: "last_bits",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[StaticParameter {
            name: "k",
            liason: ParameterLiason::Pure,
            ty: "i32",
        }],
        output_ty: "b32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(
            |values| {
                let b = values[0].take_copyable().take_b32();
                let i = values[1].take_copyable().take_i32();
                let last_bits = b & ((1 << i) - 1);
                __TempValue::Copyable(last_bits.into())
            },
            2
        )),
    },
    dev_src: __static_dev_src!(),
};
