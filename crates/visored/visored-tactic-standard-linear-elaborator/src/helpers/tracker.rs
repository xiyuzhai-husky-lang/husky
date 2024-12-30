use crate::elaborator::VdMirTacticStandardLinearElaborator;
use visored_lean_transpilation::helpers::tracker::VdLeanTranspilationTracker;

pub type VdMirTacticStandardLinearElaboratorTracker<'sess, Scheme, Input> =
    VdLeanTranspilationTracker<'sess, Scheme, Input, VdMirTacticStandardLinearElaborator<'sess>>;
