use std::{collections::HashMap, sync::Arc, time::Instant};

use super::*;
use dev_utils::__static_dev_src;
use husky_feature_eval::EvalFeature;
use husky_feature_gen::{FeatureArrivalIndicator, FeatureExpr};
use husky_trace_protocol::Label;
use print_utils::p;
use static_defn::*;
use vm::{
    Model, ModelLinkage, __EvalResult, __EvalValue, __EvalValueResult, __Linkage, __OwnedValue,
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
        linkage: __Linkage::Model(ModelLinkage(&NaiveI32)),
    },
    dev_src: __static_dev_src!(),
};

#[derive(Debug)]
struct NaiveI32;

impl Model for NaiveI32 {
    type Internal = HashMap<i32, Label>; // most likely labels

    fn train(
        &self,
        training_data: Vec<(Vec<__EvalValue<'static>>, Label)>,
    ) -> __EvalResult<Self::Internal> {
        let mut label_statics_map: HashMap<i32, HashMap<Label, usize>> = Default::default();
        for (arguments, label) in training_data {
            assert_eq!(arguments.len(), 1);
            let value = arguments[0].primitive().take_i32();
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
        arguments: &[__EvalValue<'eval>],
    ) -> __EvalValueResult<'eval> {
        let argument = arguments[0].primitive().take_i32();
        match most_likely_labels.get(&argument) {
            Some(l) => Ok(__EvalValue::Copyable((l.0 as i32).into())),
            None => {
                p!(argument);
                panic!();
                Ok(__EvalValue::Undefined)
            }
        }
    }
}
