pub mod dense;
pub mod sparse;

use crate::{builder::VdLeanTranspilationBuilder, VdTranspileToLean};
use lean_mir_expr::item_defn::LnItemDefnIdxRange;
use visored_mir_expr::stmt::VdMirStmtIdxRange;

pub trait IsVdLeanTranspilationScheme: Sized {
    type Cache: Default;

    fn transpile_vd_stmts_to_ln_defns(
        builder: &mut VdLeanTranspilationBuilder<Self>,
        stmts: VdMirStmtIdxRange,
    ) -> LnItemDefnIdxRange;
}
