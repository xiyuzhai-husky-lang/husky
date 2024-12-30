use crate::elaborator::VdMirTacticStandardLinearElaborator;
use visored_lean_transpilation::helpers::tracker::VdLeanTranspilationTracker;

pub type VdMirTacticStandardLinearElaboratorTracker<'a, Scheme, Input> =
    VdLeanTranspilationTracker<'a, Scheme, Input, VdMirTacticStandardLinearElaborator>;
