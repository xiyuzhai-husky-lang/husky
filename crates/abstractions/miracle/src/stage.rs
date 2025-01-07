use crate::*;
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MiracleStage {
    pub metrics: Vec<MiracleMetric>,
    pub max_norm: NotNan<f64>,
    pub max_heartbeats: u64,
}

impl MiracleStage {
    pub(crate) fn run<T: HasMiracle, R>(&self, t: &mut T, f: impl FnOnce(&mut T) -> R) -> R {
        assert!(
            t.miracle().is_uninitialized(),
            "miracle is already initialized: {:?}",
            t.miracle().inner
        );
        *t.miracle_mut() = Miracle {
            inner: MiracleInner::Initialized {
                state: MiracleState::new(),
                stage: self.clone(),
            },
        };
        let r = f(t);
        *t.miracle_mut() = Miracle::new_uninitialized();
        r
    }
}
