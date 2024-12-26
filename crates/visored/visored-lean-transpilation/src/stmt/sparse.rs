mod have;
mod show;

use super::*;
use crate::scheme::sparse::VdLeanTranspilationSparseScheme as Sparse;

impl<'a> VdLeanTranspilationBuilder<'a, Sparse> {
    pub(crate) fn transpile_vd_stmts_to_ln_defns(
        &mut self,
        stmts: VdMirStmtIdxRange,
    ) -> LnItemDefnIdxRange {
        let item_defns: Vec<_> = stmts
            .into_iter()
            .map(|stmt| self.build_ln_item_defn_from_vd_stmt(stmt))
            .collect();
        let source_map = self.source_map();
        let input = self.input();
        let token_storage = self.token_storage();
        let sem_sentence_range_map = self.sem_sentence_range_map();
        self.alloc_item_defns(
            item_defns,
            stmts.into_iter().map(|stmt| {
                let token_idx_range = match source_map[stmt] {
                    VdMirStmtSource::Stmt(_)
                    | VdMirStmtSource::Division(_)
                    | VdMirStmtSource::Clause(_) => return LnItemDefnComment::Void,
                    VdMirStmtSource::Sentence(sentence) => sem_sentence_range_map[sentence],
                };
                let offset_range = token_storage.token_idx_range_offset_range(token_idx_range);
                LnItemDefnComment::from_latex_source(&input[offset_range])
            }),
        )
    }
}

impl<'a> VdLeanTranspilationBuilder<'a, Sparse> {
    pub(crate) fn build_ln_item_defn_from_vd_stmt(&mut self, stmt: VdMirStmtIdx) -> LnItemDefnData {
        let db = self.db();
        match self.stmt_arena()[stmt] {
            VdMirStmtData::Block { stmts, ref meta } => {
                let defns = match *meta {
                    VdMirBlockMeta::Paragraph | VdMirBlockMeta::Sentence => stmts.to_lean(self),
                    VdMirBlockMeta::Environment(_, _, module_path)
                    | VdMirBlockMeta::Division(_, module_path) => {
                        self.with_module_path(module_path, |builder| stmts.to_lean(builder))
                    }
                };
                let meta = match *meta {
                    VdMirBlockMeta::Paragraph => LnMirItemDefnGroupMeta::Paragraph,
                    VdMirBlockMeta::Sentence => LnMirItemDefnGroupMeta::Sentence,
                    VdMirBlockMeta::Division(_, module_path) => LnMirItemDefnGroupMeta::Division(
                        vd_module_path_to_ln_namespace(module_path, db),
                    ),
                    VdMirBlockMeta::Environment(_, _, module_path) => {
                        LnMirItemDefnGroupMeta::Environment(
                            vd_module_path_to_ln_namespace(module_path, db).unwrap(),
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
            VdMirStmtData::Have { prop: formula } => self.build_have_stmt(formula),
            VdMirStmtData::Show { prop: formula } => self.build_show_stmt(formula),
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
