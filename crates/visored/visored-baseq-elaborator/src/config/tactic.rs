use super::*;
use ordered_float::NotNan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VdBsqTacticConfig {
    comm_ring: VdBsqCommRingTacticConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VdBsqCommRingTacticConfig {
    stages: Vec<NotNan<f64>>,
    max_heartbeats: u64,
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
            stages: vec![1.0, 2.0, 5.0]
                .into_iter()
                .map(|f| NotNan::new(f).unwrap())
                .collect(),
            max_heartbeats: 1000,
            product_expansion_limit: 42,
            exponential_expansion_limit: 10,
        }
    }
}

impl VdBsqCommRingTacticConfig {
    pub fn stages(&self) -> &[NotNan<f64>] {
        &self.stages
    }

    pub fn max_heartbeats(&self) -> u64 {
        self.max_heartbeats
    }

    pub fn product_expansion_limit(&self) -> usize {
        self.product_expansion_limit
    }

    pub fn exponential_expansion_limit(&self) -> usize {
        self.exponential_expansion_limit
    }
}
