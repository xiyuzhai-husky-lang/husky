use std::{collections::HashMap, sync::Arc, time::Instant};

use super::*;
use husky_dev_utils::__static_dev_src;
use husky_print_utils::p;
use husky_trace_protocol::Label;
use husky_vm_register_method::VMRegisterMethodX;
use static_defn::*;
use vm::{
    Model, __Linkage, __ModelLinkage, __Register, __RegistrableSafe, __VMResult,
    __I32_VTABLE,
};

static_mod! { naive = { naive_i32 } }

pub static NAIVE_I32_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "naive_i32",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[StaticParameter {
            name: "a",
            liason: ParameterLiason::Pure,
            ty: "i32",
        }],
        variadic_template: StaticVariadicTemplate::None,
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        linkage: __Linkage::Model(__ModelLinkage(&NaiveI32)),
    },
    dev_src: __static_dev_src!(),
};

#[derive(Debug)]
struct NaiveI32;

impl Model for NaiveI32 {
    type Internal = HashMap<i32, Label>; // most likely labels

    fn train<'eval>(
        &self,
        training_data: Vec<(Vec<__Register<'eval>>, __Register<'eval>)>,
    ) -> __VMResult<Self::Internal> {
        let mut label_statics_map: HashMap<i32, HashMap<Label, usize>> = Default::default();
        for (arguments, mut label) in training_data {
            assert_eq!(arguments.len(), 1);
            let value = arguments[0].downcast_i32();
            let label = unsafe {
                todo!()
                // label.downcast::<Label>()
            };
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
        Ok(most_likely_labels)
    }

    fn eval<'eval>(
        &self,
        most_likely_labels: &Self::Internal,
        arguments: &[__Register<'eval>],
    ) -> __VMResult<__Register<'eval>> {
        let argument = arguments[0].downcast_i32();
        match most_likely_labels.get(&argument) {
            Some(l) => Ok(l.to_register()),
            None => {
                p!(argument);
                panic!();
                Ok(__Register::new_undefined(
                    todo!(), // &__I64_VTABLE
                ))
            }
        }
    }
}
