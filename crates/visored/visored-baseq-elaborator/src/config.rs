mod tactic;

use self::tactic::VdBsqTacticConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VdBsqElaboratorConfig {
    tactic: VdBsqTacticConfig,
}

impl VdBsqElaboratorConfig {
    pub fn new_ad_hoc() -> Self {
        Self {
            tactic: VdBsqTacticConfig::new_ad_hoc(),
        }
    }
}

impl VdBsqElaboratorConfig {
    pub fn tactic(&self) -> &VdBsqTacticConfig {
        &self.tactic
    }
}
