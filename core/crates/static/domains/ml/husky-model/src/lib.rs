use husky_feature_eval::*;
use husky_feature_gen::*;
use husky_trace_protocol::*;
use std::{
    collections::HashMap,
    marker::PhantomData,
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Arc,
    time::Instant,
};
use vm::*;

// #[derive(Debug)]
// pub struct Wrapper<T: Model> {
//     phantomData: PhantomData<T>,
// }

// impl<T: Model> Model for Wrapper<T> {
//     fn train_dyn(
//         &self,
//         opt_arrival_indicator: Option<&dyn std::any::Any>,
//         opds: &dyn std::any::Any,
//     ) -> __EvalResult {
//         let opt_arrival_indicator: Option<&FeatureBranchIndicator> =
//             branch_indicator.map(|r| r.downcast_ref().unwrap());
//         todo!();
//         let opds: &Vec<Arc<FeatureExpr>> = opds.downcast_ref().unwrap();
//         assert_eq!(opds.len(), 1);
//         let opd = &opds[0];
//         let eval_time = husky_eval_time::eval_time();
//         let session = eval_time.session();
//         let dev_division = session.dev();
//         let mut label_statics_map: HashMap<i32, HashMap<Label, usize>> = Default::default();
//         let now = Instant::now();
//         for labeled_data in dev_division.each_labeled_data() {
//             let sample_id = labeled_data.sample_id;
//             if sample_id.0 >= 1000 {
//                 break;
//             }
//             let value = eval_time
//                 .eval_feature_expr(opd, sample_id)
//                 .map_err(|e| (sample_id, e))?
//                 .primitive()
//                 .take_i32();
//             *label_statics_map
//                 .entry(value)
//                 .or_default()
//                 .entry(labeled_data.label)
//                 .or_default() += 1;
//         }
//         println!(
//             "{} milliseconds elapsed for evaluating first 1000 in naive train",
//             now.elapsed().as_millis(),
//         );
//         let most_likely_labels: HashMap<i32, i32> = label_statics_map
//             .into_iter()
//             .map(|(value, label_statics)| -> (i32, i32) {
//                 (
//                     value,
//                     label_statics
//                         .into_iter()
//                         .max_by(|x, y| x.1.cmp(&y.1))
//                         .unwrap()
//                         .0
//                          .0 as i32,
//                 )
//             })
//             .collect();
//         // Ok(__EvalValue::Owned(__OwnedValue::new(most_likely_labels)));
//         // let internal = self.t.train(branch_indicator, opds);
//         Ok(todo!())
//     }

//     fn eval_dyn<'eval>(
//         &self,
//         internal: &__EvalValue<'static>,
//         arguments: &[__EvalValue<'eval>],
//     ) -> EvalValueResult<'eval> {
//         todo!()
//     }
// }
