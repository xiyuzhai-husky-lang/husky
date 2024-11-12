//! In full generality, patterns are boolean functions over a type.
//!
//! Here our `pattern` means a boolean function over a certain type in mathematics
//!
//! ```latex
//! Let $x\in \mathbb{N}$.
//! ```
//!
//! In the above, $x$ is a trivial letter pattern over the natural numbers.

use crate::{symbol::local_defn::VdHirSymbolLocalDefnIdx, *};
use latex_math_letter::letter::LxMathLetter;
use visored_sem_expr::pattern::VdSemPattern;

#[derive(Debug, PartialEq, Eq)]
pub enum VdHirPattern {
    Letter {
        letter: LxMathLetter,
        symbol_local_defn: VdHirSymbolLocalDefnIdx,
    },
}

impl ToVdHir<VdHirPattern> for &VdSemPattern {
    fn to_vd_hir(self, builder: &mut VdHirExprBuilder) -> VdHirPattern {
        match *self {
            VdSemPattern::Letter {
                token_idx_range,
                letter,
                local_defn,
            } => VdHirPattern::Letter {
                letter,
                symbol_local_defn: local_defn.to_vd_hir(builder),
            },
        }
    }
}
