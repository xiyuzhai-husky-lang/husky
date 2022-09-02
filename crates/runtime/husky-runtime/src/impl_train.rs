use crate::*;
use husky_feature_gen::{
    FeatureArrivalIndicator, FeatureLazyExpr, FeatureLazyExprVariant, TrainModel,
};
use husky_vm::{GenericArgument, __Register, __VMResult};
use husky_vm::{__RegistrableSafe, __VirtualEnum};
use std::time::Instant;

impl TrainModel for HuskyRuntime {
    fn train(
        &self,
        model: husky_vm::__ModelLinkage,
        opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
        opds: &[Arc<FeatureLazyExpr>],
    ) -> husky_vm::__VMResult<__Register<'static>> {
        let session = self.session();
        let dev_division = session.dev();
        let mut arguments: Vec<GenericArgument> = opds
            .iter()
            .map(|opd| match opd.variant {
                FeatureLazyExprVariant::Literal(ref register) => GenericArgument::Literal {
                    value: register.clone(),
                },
                _ => GenericArgument::NonConstant { values: vec![] },
            })
            .collect();
        let mut labels: Vec<i32> = vec![];
        for labeled_data in dev_division.each_labeled_data() {
            let sample_id = labeled_data.sample_id;
            if !self.eval_opt_arrival_indicator_cached(opt_arrival_indicator, sample_id)? {
                continue;
            }
            for (opd, argument) in std::iter::zip(opds.iter(), arguments.iter_mut()) {
                match argument {
                    GenericArgument::NonConstant { values } => {
                        values.push(self.eval_feature_expr_cached(opd, sample_id).map_err(|e| {
                            todo!()
                            // (sample_id, e)
                        })?)
                    }
                    GenericArgument::Literal { .. } => (),
                }
            }
            labels.push(labeled_data.label.0)
        }
        let train_result = model.train_dyn(arguments, labels);
        train_result
    }
}
