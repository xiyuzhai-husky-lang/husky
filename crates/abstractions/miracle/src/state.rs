use crate::*;
use ordered_float::NotNan;

pub struct MiracleState {
    vector: Vec<u64>,
}

impl MiracleState {
    pub fn new() -> Self {
        Self { vector: vec![] }
    }
}

impl MiracleState {
    pub fn norm(&self, metrics: &[MiracleMetric]) -> NotNan<f64> {
        metrics
            .iter()
            .map(|metric| metric.norm(&self.vector))
            .min()
            .expect("metrics should not be empty")
    }
}

impl Miracle {
    pub(crate) fn with_new_value_appended<R>(
        &mut self,
        value: u64,
        mut f: impl FnMut(&mut Self) -> R,
    ) -> R {
        self.state_mut().vector.push(value);
        let result = f(self);
        self.state_mut().vector.pop();
        result
    }
}
