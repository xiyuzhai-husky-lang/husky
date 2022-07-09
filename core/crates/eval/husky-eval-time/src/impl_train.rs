use crate::*;
use husky_entity_semantics::StoreEntityRoute;
use husky_feature_gen::{FeatureBranchIndicator, FeatureExpr, TrainModel};
use std::time::Instant;
use upcast::Upcast;
use vm::{InterpreterQueryGroup, VMConfig, __EvalResult, __EvalValue, __OwnedValue};

impl TrainModel for HuskyEvalTime {
    fn train(
        &self,
        model: vm::ModelLinkage,
        opt_branch_indicator: Option<&Arc<FeatureBranchIndicator>>,
        opds: &[Arc<FeatureExpr>],
    ) -> vm::__EvalResult {
        if let Some(branch_indicator) = opt_branch_indicator {
            todo!()
        }
        const MAX_SAMPLE_LEN: usize = 1000;
        let session = self.session();
        let dev_division = session.dev();
        let mut label_statics_map: HashMap<i32, HashMap<Label, usize>> = Default::default();
        let now = Instant::now();
        let mut training_data: Vec<(Vec<__EvalValue>, Label)> = Vec::new();
        for labeled_data in dev_division.each_labeled_data() {
            let sample_id = labeled_data.sample_id;
            if sample_id.0 >= MAX_SAMPLE_LEN {
                break;
            }
            let values: Vec<__EvalValue> = opds
                .iter()
                .map(|opd| self.husky_feature_eval_expr(opd, sample_id))
                .collect::<__EvalResult<Vec<_>>>()
                .map_err(|e| (sample_id, e))?;
            training_data.push((values, labeled_data.label))
        }
        let internal = model.train_dyn(training_data);
        println!(
            "{} milliseconds elapsed for evaluating first 1000 in naive train",
            now.elapsed().as_millis(),
        );
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
        Ok(__EvalValue::Owned(__OwnedValue::new(most_likely_labels)))
    }
}
