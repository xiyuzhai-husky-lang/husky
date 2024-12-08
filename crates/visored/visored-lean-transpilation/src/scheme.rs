use crate::{stmt::IsSchemeForTranspileVdStmtsToLnDefns, VdTranspileToLean};
use lean_mir_expr::item_defn::LnItemDefnIdxRange;
use visored_mir_expr::stmt::VdMirStmtIdxRange;

pub trait IsVdLeanTranspilationScheme: IsSchemeForTranspileVdStmtsToLnDefns {}

pub struct VdLeanTranspilationDenseScheme;

impl IsVdLeanTranspilationScheme for VdLeanTranspilationDenseScheme {}

pub struct VdLeanTranspilationSparseScheme;

impl IsVdLeanTranspilationScheme for VdLeanTranspilationSparseScheme {}
