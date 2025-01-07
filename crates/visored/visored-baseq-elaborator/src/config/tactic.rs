use super::*;
use miracle::{metric::MiracleMetric, stage::MiracleStage};
use ordered_float::NotNan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VdBsqTacticConfig {
    comm_ring: VdBsqCommRingTacticConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VdBsqCommRingTacticConfig {
    stages: Vec<MiracleStage>,
    product_expansion_limit: usize,
    exponential_expansion_limit: usize,
}

impl VdBsqTacticConfig {
    pub fn new_ad_hoc() -> Self {
        Self {
            comm_ring: VdBsqCommRingTacticConfig::new_ad_hoc(),
        }
    }
}

impl VdBsqTacticConfig {
    pub fn comm_ring(&self) -> &VdBsqCommRingTacticConfig {
        &self.comm_ring
    }
}

impl VdBsqCommRingTacticConfig {
    pub fn new_ad_hoc() -> Self {
        Self {
            stages: vec![
                MiracleStage {
                    max_norm: NotNan::new(2.0).unwrap(),
                    max_heartbeats: 100,
                    metrics: vec![MiracleMetric::L1 {
                        scale: NotNan::new(1.0).unwrap(),
                    }],
                },
                MiracleStage {
                    max_norm: NotNan::new(5.0).unwrap(),
                    max_heartbeats: 1000,
                    metrics: vec![MiracleMetric::L1 {
                        scale: NotNan::new(1.0).unwrap(),
                    }],
                },
            ],
            product_expansion_limit: 42,
            exponential_expansion_limit: 10,
        }
    }
}

impl VdBsqCommRingTacticConfig {
    pub fn stages(&self) -> &[MiracleStage] {
        &self.stages
    }

    pub fn product_expansion_limit(&self) -> usize {
        self.product_expansion_limit
    }

    pub fn exponential_expansion_limit(&self) -> usize {
        self.exponential_expansion_limit
    }
}
