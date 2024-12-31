use crate::elaborator::VdMirStandardSequentialElaborator;
use visored_lean_transpilation::helpers::tracker::VdLeanTranspilationTracker;

pub type VdMirStandardSequentialElaboratorTracker<'sess, Scheme, Input> =
    VdLeanTranspilationTracker<'sess, Scheme, Input, VdMirStandardSequentialElaborator<'sess>>;
