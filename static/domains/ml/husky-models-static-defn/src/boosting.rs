use husky_vm::GenericArgument;
use ordered_float::NotNan;

use crate::__rust_code_gen__::__BOOSTING_WITH_VMAX_NORMALIZED_INTERNAL_VTABLE;

use super::*;

static_mod! { boosting = { boosting_with_vmax_normalized } }

pub static BOOSTING_WITH_VMAX_NORMALIZED_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "boosting_with_vmax_normalized",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[StaticParameter {
            name: "label0",
            modifier: ParameterModifier::None,
            ty: "TargetOutput",
        }],
        variadic_template: StaticVariadicParameterDecl::RepeatSingle(StaticParameter {
            name: "args",
            modifier: ParameterModifier::None,
            ty: "f32",
        }),
        return_ty: "TargetOutput",
        output_liason: OutputModifier::Transfer,
        linkage: __LinkageGroup::Model(__ModelLinkageGroup(&BoostingWithVmaxNormalized)),
    },
    dev_src: static_dev_src!(),
};

#[derive(Debug)]
struct BoostingWithVmaxNormalized;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BoostingWithVmaxNormalizedInternal {
    label0: i32,
    vmaxs: Vec<NotNan<f32>>,
    threshold: NotNan<f32>,
}

impl __StaticInfo for BoostingWithVmaxNormalizedInternal {
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

impl<'eval> __Registrable<'eval> for BoostingWithVmaxNormalizedInternal {
    unsafe fn __to_register(self) -> __Register<'eval> {
        __Register::new_box(self, &__BOOSTING_WITH_VMAX_NORMALIZED_INTERNAL_VTABLE)
    }
}

impl Model for BoostingWithVmaxNormalized {
    type Internal = BoostingWithVmaxNormalizedInternal;

    fn internal_ty_vtable() -> &'static husky_vm::__RegisterTyVTable {
        &__BOOSTING_WITH_VMAX_NORMALIZED_INTERNAL_VTABLE
    }

    fn train<'eval>(
        &self,
        opds: Vec<GenericArgument>,
        labels: Vec<i32>,
    ) -> __VMResult<Self::Internal> {
        let label0: &__VirtualEnum = opds[0].value().downcast_temp_ref(&__VIRTUAL_ENUM_VTABLE);
        let label0 = label0.kind_idx;
        let inputs: Vec<Vec<NotNan<f32>>> = (1..opds.len())
            .into_iter()
            .map(|i| {
                opds[i]
                    .values()
                    .iter()
                    .map(|r| {
                        let val = r.downcast_f32();
                        assert!(val >= 0.);
                        NotNan::new(val).expect("todo")
                    })
                    .collect()
            })
            .collect();
        let vmaxs: Vec<NotNan<f32>> = inputs
            .iter()
            .map(|input| {
                *input
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| if labels[i] == label0 { Some(v) } else { None })
                    .max()
                    .expect(&format!("input.len() = {}, todo", input.len()))
            })
            .collect();
        let mut true_weighted_sums: Vec<NotNan<f32>> = labels
            .iter()
            .enumerate()
            .filter_map(|(i, label)| {
                if *label == label0 {
                    Some(
                        std::iter::zip(vmaxs.iter(), inputs.iter().map(|input| input[i]))
                            .map(|(vmax, input)| input / vmax)
                            .sum(),
                    )
                } else {
                    None
                }
            })
            .collect();
        true_weighted_sums.sort();
        let mut false_weighted_sums: Vec<NotNan<f32>> = labels
            .iter()
            .enumerate()
            .filter_map(|(i, label)| {
                if *label != label0 {
                    Some(
                        std::iter::zip(vmaxs.iter(), inputs.iter().map(|input| input[i]))
                            .map(|(vmax, input)| input / vmax)
                            .sum(),
                    )
                } else {
                    None
                }
            })
            .collect();
        false_weighted_sums.sort();
        let mut i = 0;
        while i < true_weighted_sums.len() && i < false_weighted_sums.len() {
            if true_weighted_sums[true_weighted_sums.len() - 1 - i] < false_weighted_sums[i] {
                break;
            }
            i += 1
        }
        let threshold =
            (true_weighted_sums[true_weighted_sums.len() - 1 - i] + false_weighted_sums[i]) / 2.;
        Ok(BoostingWithVmaxNormalizedInternal {
            label0,
            vmaxs,
            threshold,
        })
    }

    fn eval<'eval>(
        &self,
        internal: &Self::Internal,
        arguments: &[__Register<'eval>],
    ) -> __VMResult<__Register<'eval>> {
        let sum: NotNan<f32> = NotNan::new(
            (1..arguments.len())
                .into_iter()
                .map(|i| arguments[i].downcast_f32() / internal.vmaxs[i - 1].into_inner())
                .sum(),
        )
        .unwrap();
        Ok(if sum > internal.threshold {
            __Register::none(0)
        } else {
            __VirtualEnum {
                kind_idx: internal.label0,
            }
            .to_register()
        })
    }
}
