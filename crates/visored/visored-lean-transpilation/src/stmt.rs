mod then;

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

impl VdTranspileToLean<LnItemDefnIdxRange> for VdMirStmtIdxRange {
    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder) -> LnItemDefnIdxRange {
        let item_defns: Vec<_> = self
            .into_iter()
            .map(|stmt| builder.build_ln_item_defn_from_vd_stmt(stmt))
            .collect();
        let source_map = builder.source_map();
        let input = builder.input();
        let token_storage = builder.token_storage();
        let sem_expr_range_map = builder.sem_expr_range_map();
        let sem_phrase_range_map = builder.sem_phrase_range_map();
        let sem_clause_range_map = builder.sem_clause_range_map();
        let sem_sentence_range_map = builder.sem_sentence_range_map();
        let sem_division_range_map = builder.sem_division_range_map();
        let sem_stmt_range_map = builder.sem_stmt_range_map();
        builder.alloc_item_defns(
            item_defns,
            self.into_iter().map(|stmt| {
                let token_idx_range = match source_map[stmt] {
                    VdMirStmtSource::Stmt(_)
                    | VdMirStmtSource::Division(_)
                    | VdMirStmtSource::Sentence(_) => return LnItemDefnComment::Void,
                    VdMirStmtSource::Clause(clause) => sem_clause_range_map[clause],
                };
                let offset_range = token_storage.token_idx_range_offset_range(token_idx_range);
                LnItemDefnComment::from_latex_source(&input[offset_range])
            }),
        )
    }
}

impl<'a> VdLeanTranspilationBuilder<'a> {
    pub(crate) fn build_ln_item_defn_from_vd_stmt(&mut self, stmt: VdMirStmtIdx) -> LnItemDefnData {
        match self.stmt_arena()[stmt] {
            VdMirStmtData::Block { stmts, ref meta } => {
                let defns = match *meta {
                    VdMirBlockMeta::Paragraph | VdMirBlockMeta::Sentence => stmts.to_lean(self),
                    VdMirBlockMeta::Environment(_, module_path)
                    | VdMirBlockMeta::Division(_, module_path) => {
                        self.with_module_path(module_path, |builder| stmts.to_lean(builder))
                    }
                };
                let meta = match *meta {
                    VdMirBlockMeta::Paragraph => LnMirItemDefnGroupMeta::Paragraph,
                    VdMirBlockMeta::Sentence => LnMirItemDefnGroupMeta::Sentence,
                    VdMirBlockMeta::Division(_, module_path) => LnMirItemDefnGroupMeta::Division(
                        *vd_module_path_to_ln_namespace(module_path),
                    ),
                    VdMirBlockMeta::Environment(_, module_path) => {
                        LnMirItemDefnGroupMeta::Environment(
                            vd_module_path_to_ln_namespace(module_path).unwrap(),
                        )
                    }
                };
                LnItemDefnData::Group { defns, meta }
            }
            VdMirStmtData::LetPlaceholder { ref pattern, ty } => {
                self.build_ln_item_from_vd_let_placeholder_stmt(pattern, ty)
            }
            VdMirStmtData::LetAssigned {
                ref pattern,
                assignment,
            } => todo!(),
            VdMirStmtData::Then { formula } => self.build_then_stmt(formula),
        }
    }

    fn build_ln_item_from_vd_let_placeholder_stmt(
        &mut self,
        pattern: &VdMirPattern,
        ty: VdMirExprIdx,
    ) -> LnItemDefnData {
        let ident = match *pattern {
            VdMirPattern::Letter {
                symbol_local_defn, ..
            } => self.mangle_symbol(symbol_local_defn),
            VdMirPattern::Assumed => self.mangle_hypothesis(),
        };
        LnItemDefnData::Variable {
            ident,
            ty: ty.to_lean(self),
        }
    }
}
