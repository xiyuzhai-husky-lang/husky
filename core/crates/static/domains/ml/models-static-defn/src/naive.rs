use std::{collections::HashMap, sync::Arc};

use super::*;
use dev_utils::static_dev_src;
use eval_feature::EvalFeature;
use feature_gen::FeatureLazyExpr;
use husky_tracer_protocol::Label;
use static_defn::*;
use vm::{EvalResult, EvalValue, ModelLinkage, OwnedValue};

static_mod! { naive = { naive_i32 } }

pub static NAIVE_I32_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "naive_i32",
    items: &[],
    variant: EntityStaticDefnVariant::Model {
        spatial_parameters: &[],
        parameters: &[StaticParameter {
            name: "a",
            liason: ParameterLiason::Pure,
            ty: "i32",
        }],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        Model_variant: StaticModelVariant::Model(ModelLinkage {
            train: naive_i32_train,
            eval: naive_i32_eval,
        }),
    },
    dev_src: static_dev_src!(),
};

fn naive_i32_train(opds: &dyn std::any::Any) -> EvalResult<'static> {
    let opds: &Vec<Arc<FeatureLazyExpr>> = opds.downcast_ref().unwrap();
    assert_eq!(opds.len(), 1);
    let opd = &opds[0];
    let eval_time = husky_eval_time::husky_eval_time();
    let session = eval_time.session();
    let dev_division = session.dev();
    let mut label_statics_map: HashMap<i32, HashMap<Label, usize>> = Default::default();
    for labeled_data in dev_division.each_labeled_data() {
        let sample_id = labeled_data.sample_id;
        if sample_id.0 >= 50 {
            break;
        }
        let value = eval_time
            .eval_feature_lazy_expr(opd, sample_id)
            .expect("todos")
            .primitive()
            .take_i32();
        *label_statics_map
            .entry(value)
            .or_default()
            .entry(labeled_data.label)
            .or_default() += 1;
    }
    let most_likely_labels: HashMap<i32, i32> = label_statics_map
        .into_iter()
        .map(|(value, label_statics)| -> (i32, i32) {
            (
                value,
                label_statics
                    .into_iter()
                    .max_by(|x, y| x.1.cmp(&y.1))
                    .unwrap()
                    .0
                     .0 as i32,
            )
        })
        .collect();
    Ok(EvalValue::Owned(OwnedValue::new(most_likely_labels)))
}

fn naive_i32_eval<'eval>(internal: &EvalValue, args: Vec<EvalValue<'eval>>) -> EvalResult<'eval> {
    let most_likely_labels: &HashMap<i32, i32> = internal.any_ref().downcast_ref();
    match most_likely_labels.get(&args[0].primitive().take_i32()) {
        Some(l) => Ok(EvalValue::Copyable((*l).into())),
        None => Ok(EvalValue::Undefined),
    }
}
