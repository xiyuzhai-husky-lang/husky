use crate::elaborator::VdBsqElaborator;
use visored_lean_transpilation::helpers::tracker::VdLeanTranspilationTracker;

pub type VdBsqElaboratorTracker<'sess, Scheme, Input> =
    VdLeanTranspilationTracker<'sess, Scheme, Input>;
