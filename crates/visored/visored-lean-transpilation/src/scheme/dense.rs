use super::*;

pub struct VdLeanTranspilationDenseScheme;

#[derive(Default)]
pub struct VdLeanTranspilationDenseCache {}

impl IsVdLeanTranspilationScheme for VdLeanTranspilationDenseScheme {
    type Cache = VdLeanTranspilationDenseCache;

    fn transpile_vd_stmts_to_ln_defns(
        builder: &mut VdLeanTranspilationBuilder<Self>,
        stmts: VdMirStmtIdxRange,
    ) -> LnItemDefnIdxRange {
        builder.transpile_vd_stmts_to_ln_defns(stmts)
    }
}
