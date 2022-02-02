use vm::EvalValue;

use crate::{cache::CacheFeature, eval::Evaluator, intern::InternFeature, *};

#[salsa::query_group(FeatureQueryStorage)]
pub trait FeatureSalsaQuery: InternFeature {
    fn main_feature(&self) -> FeatureId {}
}

fn main_feature(this: &dyn FeatureSalsaQuery) -> FeatureId {
    todo!()
}

// pub trait FeatureQuery: FeatureSalsaQuery {
//     fn eval(&self, feature: FeatureId, idx: usize) -> EvalValue<'static, 'static> {
//         let features = self.features();
//         let cache = self.cache(idx);
//         Evaluator::<'_, 'static>::new(features, cache).eval(feature)
//     }
// }
