mod tactic;

use self::tactic::VdBsqTacticConfig;
use miracle::{metric::MiracleMetric, stage::MiracleStage};
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VdBsqElaboratorConfig {
    stages: Vec<MiracleStage>,
    tactic: VdBsqTacticConfig,
}

impl VdBsqElaboratorConfig {
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
            tactic: VdBsqTacticConfig::new_ad_hoc(),
        }
    }
}

impl VdBsqElaboratorConfig {
    pub fn stages(&self) -> &[MiracleStage] {
        &self.stages
    }

    pub fn tactic(&self) -> &VdBsqTacticConfig {
        &self.tactic
    }
}
