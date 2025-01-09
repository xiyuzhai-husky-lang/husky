mod dense;
mod have;
mod qed;
mod sparse;

use crate::*;
use lean_mir_expr::{
    expr::LnMirExprData,
    item_defn::{LnItemDefnComment, LnItemDefnData, LnItemDefnIdxRange, LnMirItemDefnGroupMeta},
};
use namespace::vd_module_path_to_ln_namespace;
use ty::VdTypeLeanTranspilation;
use visored_mir_expr::{
    expr::{VdMirExprData, VdMirExprIdx},
    pattern::VdMirPattern,
    stmt::{
        block::VdMirBlockMeta, VdMirStmtData, VdMirStmtIdx, VdMirStmtIdxRange, VdMirStmtSource,
    },
};
use visored_term::ty::VdType;

impl<Scheme> VdTranspileToLean<Scheme, LnItemDefnIdxRange> for VdMirStmtIdxRange
where
    Scheme: IsVdLeanTranspilationScheme,
{
    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder<Scheme>) -> LnItemDefnIdxRange {
        Scheme::transpile_vd_stmts_to_ln_defns(builder, self)
    }
}
