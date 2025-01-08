pub mod stack;

use crate::{
    elaborator::VdBsqElaboratorInner, maneuver::VdBsqManeuverCall, strategy::VdBsqStrategyCall,
    tactic::VdBsqTacticCall,
};

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqCall {
    Strategy(VdBsqStrategyCall),
    Tactic(VdBsqTacticCall),
    Maneuver(VdBsqManeuverCall),
}
