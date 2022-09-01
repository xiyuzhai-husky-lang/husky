use crate::__rust_code_gen__::__NORMALIZE_VMAX_F_32_INTERNAL_VTABLE;

use super::*;

static_mod! { scaling = { normalize_vmax_f32 } }

pub static NORMALIZE_VMAX_F32_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "naive_i32",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[StaticParameter {
            name: "a",
            modifier: ParameterModifier::None,
            ty: "i32",
        }],
        variadic_template: StaticVariadicTemplate::None,
        output_ty: "CrateOutput",
        output_liason: OutputModifier::Transfer,
        linkage: __Linkage::Model(__ModelLinkage(&NormalizeVmaxF32)),
    },
    dev_src: static_dev_src!(),
};

#[derive(Debug)]
struct NormalizeVmaxF32;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NormalizeVmaxF32Internal {
    k: ordered_float::NotNan<f32>,
}

impl __StaticInfo for NormalizeVmaxF32Internal {
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

impl<'eval> __Registrable<'eval> for NormalizeVmaxF32Internal {
    unsafe fn __to_register(self) -> __Register<'eval> {
        __Register::new_box(self, &__NORMALIZE_VMAX_F_32_INTERNAL_VTABLE)
    }
}

impl Model for NormalizeVmaxF32 {
    type Internal = NormalizeVmaxF32Internal; // most likely labels

    fn train<'eval>(
        &self,
        training_data: Vec<(Vec<__Register<'eval>>, __Register<'eval>)>,
    ) -> __VMResult<Self::Internal> {
        todo!()
        // let mut label_statics_map: HashMap<i32, HashMap<Label, usize>> = Default::default();
        // for (arguments, mut label) in training_data {
        //     assert_eq!(arguments.len(), 1);
        //     let value = arguments[0].downcast_i32();
        //     let label: &__VirtualEnum = label.downcast_temp_ref(&__VIRTUAL_ENUM_VTABLE);
        //     let label = Label(label.kind_idx);
        //     *label_statics_map
        //         .entry(value)
        //         .or_default()
        //         .entry(label)
        //         .or_default() += 1;
        // }
        // let most_likely_labels: HashMap<i32, Label> = label_statics_map
        //     .into_iter()
        //     .map(|(value, label_statics)| -> (i32, Label) {
        //         (
        //             value,
        //             label_statics
        //                 .into_iter()
        //                 .max_by(|x, y| x.1.cmp(&y.1))
        //                 .unwrap()
        //                 .0,
        //         )
        //     })
        //     .collect();
        // Ok(NormalizeVmaxF32Internal { most_likely_labels })
    }

    fn eval<'eval>(
        &self,
        internal: &Self::Internal,
        arguments: &[__Register<'eval>],
    ) -> __VMResult<__Register<'eval>> {
        Ok((arguments[0].downcast_f32() * internal.k.into_inner()).to_register())
    }

    fn internal_ty_vtable() -> &'static husky_vm::__RegisterTyVTable {
        &__NORMALIZE_VMAX_F_32_INTERNAL_VTABLE
    }
}
