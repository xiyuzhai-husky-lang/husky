use crate::elaborator::VdBaseqElaborator;
use visored_lean_transpilation::helpers::tracker::VdLeanTranspilationTracker;

pub type VdBaseqElaboratorTracker<'sess, Scheme, Input> =
    VdLeanTranspilationTracker<'sess, Scheme, Input, VdBaseqElaborator<'sess>>;
