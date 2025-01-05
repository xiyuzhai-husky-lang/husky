use ordered_float::NotNan;

pub enum MiracleMetric {
    L1 { scale: f64 },
}

impl MiracleMetric {
    pub fn norm(&self, vector: &[u64]) -> NotNan<f64> {
        match self {
            MiracleMetric::L1 { scale } => {
                NotNan::new(vector.iter().map(|&v| v as f64).sum::<f64>() * scale)
                    .unwrap_or_else(|_| NotNan::new(f64::INFINITY).unwrap())
            }
        }
    }
}
