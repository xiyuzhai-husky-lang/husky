use husky_vm::GenericArgument;
use ordered_float::NotNan;

use crate::{
    __rust_code_gen__::__NARROW_DOWN_INTERNAL_VTABLE,
    utils::{FlagRange, FlagVectorField},
};

use super::*;

static_mod! { narrow = { narrow_down } }

pub static NARROW_DOWN_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "narrow_down",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[
            StaticParameter {
                name: "label0",
                modifier: ParameterModifier::None,
                ty: "TargetOutput",
            },
            StaticParameter {
                name: "ntrim",
                modifier: ParameterModifier::None,
                ty: "i32",
            },
        ],
        variadic_template: StaticVariadicParameterDecl::RepeatSingle(StaticParameter {
            name: "args",
            modifier: ParameterModifier::None,
            ty: "f32",
        }),
        return_ty: "?TargetOutput",
        output_liason: OutputModifier::Transfer,
        linkage: __LinkageGroup::Model(__ModelLinkageGroup(&NarrowDown)),
    },
    dev_src: static_dev_src!(),
};

#[derive(Debug)]
struct NarrowDown;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NarrowDownInternal {
    label0: i32,
    opt_flag_ranges: Option<Vec<FlagRange>>,
}

impl __StaticInfo for NarrowDownInternal {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        std::mem::transmute(self)
    }
}

impl<'eval> __Registrable<'eval> for NarrowDownInternal {
    unsafe fn __to_register(self) -> __Register<'eval> {
        __Register::new_box(self, &__NARROW_DOWN_INTERNAL_VTABLE)
    }
}

impl Model for NarrowDown {
    type Internal = NarrowDownInternal;

    fn internal_ty_vtable() -> &'static husky_vm::__RegisterTyVTable {
        &__NARROW_DOWN_INTERNAL_VTABLE
    }

    fn train<'eval>(
        &self,
        opds: Vec<GenericArgument>,
        labels: Vec<i32>,
    ) -> __VMResult<Self::Internal> {
        let fvf = FlagVectorField::from_registers(&opds[0], &opds[2..], &labels)?;
        let ntrim = opds[1].value().downcast_i32();
        Ok(NarrowDownInternal {
            label0: fvf.label0(),
            opt_flag_ranges: fvf.flag_ranges(ntrim, 0.1),
        })
    }

    fn eval<'eval>(
        &self,
        internal: &Self::Internal,
        arguments: &[__Register<'eval>],
    ) -> __VMResult<__Register<'eval>> {
        if let Some(ref flag_ranges) = internal.opt_flag_ranges {
            for (argument, flag_range) in std::iter::zip(arguments[2..].iter(), flag_ranges.iter())
            {
                let v = argument.downcast_f32();
                let v = NotNan::new(v).unwrap();
                let apply_result = flag_range.apply(v);
                if !apply_result.within_false_range() && apply_result.within_true_range() {
                    return Ok(__VirtualEnum {
                        kind_idx: internal.label0,
                    }
                    .to_register());
                } else if apply_result.within_false_range() && !apply_result.within_true_range() {
                    // corresponds to `return Some(None)` in Rust
                    return Ok(__Register::none(1));
                }
            }
        }
        Ok(__Register::none(0))
    }
}
