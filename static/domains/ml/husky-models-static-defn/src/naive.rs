use husky_vm::GenericArgument;

use crate::__rust_code_gen__::__NAIVE_I_32_INTERNAL_VTABLE;

use super::*;

static_mod! { naive = { naive_i32 } }

pub static NAIVE_I32_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "naive_i32",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[StaticParameter {
            name: "a",
            modifier: ParameterModifier::None,
            ty: "i32",
        }],
        variadic_template: StaticVariadicParameterDecl::None,
        return_ty: "TargetOutput",
        output_liason: OutputModifier::Transfer,
        linkage: __Linkage::Model(__ModelLinkage(&NaiveI32)),
    },
    dev_src: static_dev_src!(),
};

#[derive(Debug)]
struct NaiveI32;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NaiveI32Internal {
    most_likely_labels: HashMap<i32, Label>,
}

impl __StaticInfo for NaiveI32Internal {
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

impl<'eval> __Registrable<'eval> for NaiveI32Internal {
    unsafe fn __to_register(self) -> __Register<'eval> {
        __Register::new_box(self, &__NAIVE_I_32_INTERNAL_VTABLE)
    }
}

impl Model for NaiveI32 {
    type Internal = NaiveI32Internal; // most likely labels

    fn train<'eval>(
        &self,
        opds: Vec<GenericArgument>,
        labels: Vec<i32>,
    ) -> __VMResult<Self::Internal> {
        let mut label_statics_map: HashMap<i32, HashMap<Label, usize>> = Default::default();
        for (value, label) in std::iter::zip(opds[0].values().iter(), labels.iter()) {
            let value = value.downcast_i32();
            let label = Label(*label);
            *label_statics_map
                .entry(value)
                .or_default()
                .entry(label)
                .or_default() += 1;
        }
        let most_likely_labels: HashMap<i32, Label> = label_statics_map
            .into_iter()
            .map(|(value, label_statics)| -> (i32, Label) {
                (
                    value,
                    label_statics
                        .into_iter()
                        .max_by(|x, y| x.1.cmp(&y.1))
                        .unwrap()
                        .0,
                )
            })
            .collect();
        Ok(NaiveI32Internal { most_likely_labels })
    }

    fn eval<'eval>(
        &self,
        internal: &Self::Internal,
        arguments: &[__Register<'eval>],
    ) -> __VMResult<__Register<'eval>> {
        let argument = arguments[0].downcast_i32();
        match internal.most_likely_labels.get(&argument) {
            Some(l) => Ok(__VirtualEnum { kind_idx: l.0 }.to_register()),
            None => Ok(__Register::none(0)),
        }
    }

    fn internal_ty_vtable() -> &'static husky_vm::__RegisterTyVTable {
        &__NAIVE_I_32_INTERNAL_VTABLE
    }
}
