pub mod letter;
pub mod punctuation;

use expr::VdSynExprArenaRef;

use super::*;
use crate::expr::VdSynExprMap;

pub enum VdSynSymbolResolution {}

pub struct VdSynSymbolResolutions {
    expr_resolutions: VdSynExprMap<VdSynSymbolResolution>,
}

impl VdSynSymbolResolutions {
    pub fn new(expr_arena: VdSynExprArenaRef) -> Self {
        Self {
            expr_resolutions: VdSynExprMap::new2(expr_arena),
        }
    }
}
