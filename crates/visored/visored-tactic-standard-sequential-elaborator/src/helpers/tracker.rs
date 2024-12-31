use crate::elaborator::VdMirTacticStandardSequentialElaborator;
use visored_lean_transpilation::helpers::tracker::VdLeanTranspilationTracker;

pub type VdMirTacticStandardSequentialElaboratorTracker<'sess, Scheme, Input> =
    VdLeanTranspilationTracker<'sess, Scheme, Input, VdMirTacticStandardSequentialElaborator<'sess>>;
