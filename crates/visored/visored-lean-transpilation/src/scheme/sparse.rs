use super::*;

pub struct VdLeanTranspilationSparseScheme;

impl IsVdLeanTranspilationScheme for VdLeanTranspilationSparseScheme {
    type Cache = ();

    fn transpile_vd_stmts_to_ln_defns(
        builder: &mut VdLeanTranspilationBuilder<Self>,
        stmts: VdMirStmtIdxRange,
    ) -> LnItemDefnIdxRange {
        builder.transpile_vd_stmts_to_ln_defns(stmts)
    }
}
