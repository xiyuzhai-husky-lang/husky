pub mod letter;
pub mod punctuation;

use expr::VdSynExprArenaRef;

use super::*;
use crate::expr::VdSynExprMap;

pub enum VdSynSymbolResolution {}

pub struct VdSynSymbolResolutionTable {
    expr_resolutions: VdSynExprMap<VdSynSymbolResolution>,
}

impl VdSynSymbolResolutionTable {
    pub fn new(expr_arena: VdSynExprArenaRef) -> Self {
        Self {
            expr_resolutions: VdSynExprMap::new2(expr_arena),
        }
    }
}
