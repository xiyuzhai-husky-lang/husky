use lean_mir_expr::item_defn::def::{LnDefParameter, LnMirDefBody};
use lean_term::ty::LnType;
use visored_entity_path::{environment::VdEnvironmentPath, module::VdModulePath};

use super::*;

impl<'a> VdLeanTranspilationBuilder<'a, Dense> {
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

impl<'a> VdLeanTranspilationBuilder<'a, Dense> {
    pub(crate) fn build_ln_item_defn_from_vd_stmt(&mut self, stmt: VdMirStmtIdx) -> LnItemDefnData {
        let db = self.db();
        match self.stmt_arena()[stmt] {
            VdMirStmtData::Block { stmts, ref meta } => {
                match *meta {
                    VdMirBlockMeta::Paragraph => self.build_ln_def_from_vd_paragraph(stmts),
                    VdMirBlockMeta::Environment(_, environment_path, module_path) => {
                        self.build_ln_def_from_vd_environment(stmts, environment_path, module_path)
                    }
                    VdMirBlockMeta::Division(_, module_path) => {
                        let defns =
                            self.with_module_path(module_path, |builder| stmts.to_lean(builder));
                        LnItemDefnData::Group {
                            defns,
                            meta: LnMirItemDefnGroupMeta::Division(vd_module_path_to_ln_namespace(
                                module_path,
                                db,
                            )),
                        }
                    }
                    VdMirBlockMeta::Sentence => unreachable!(),
                }
                // let defns = match *meta {
                //     VdMirBlockMeta::Paragraph => todo!(),
                //     VdMirBlockMeta::Environment(_, environment_path, module_path) => todo!(),
                //     VdMirBlockMeta::Division(_, module_path) => {
                //         self.with_module_path(module_path, |builder| stmts.to_lean(builder))
                //     }
                //     VdMirBlockMeta::Sentence => unreachable!(),
                // };
                // let meta = match *meta {
                //     VdMirBlockMeta::Paragraph => LnMirItemDefnGroupMeta::Paragraph,
                //     VdMirBlockMeta::Sentence => LnMirItemDefnGroupMeta::Sentence,
                //     VdMirBlockMeta::Division(_, module_path) => LnMirItemDefnGroupMeta::Division(
                //         vd_module_path_to_ln_namespace(module_path, db),
                //     ),
                //     VdMirBlockMeta::Environment(_, environment_path, module_path) => {
                //         todo!();
                //         LnMirItemDefnGroupMeta::Environment(
                //             vd_module_path_to_ln_namespace(module_path, db).unwrap(),
                //         )
                //     }
                // };
                // LnItemDefnData::Group { defns, meta }
            }
            VdMirStmtData::LetPlaceholder { .. }
            | VdMirStmtData::LetAssigned { .. }
            | VdMirStmtData::Then { .. } => unreachable!(),
        }
    }

    fn build_ln_def_from_vd_paragraph(&mut self, stmts: VdMirStmtIdxRange) -> LnItemDefnData {
        let ident = self.mangle_hypothesis();
        let parameters = stmts
            .into_iter()
            .map_while(|stmt| self.build_ln_parameter_from_vd_stmt(stmt))
            .collect();
        LnItemDefnData::Def {
            ident,
            parameters,
            ty: None,
            body: LnMirDefBody::Tactics(stmts.to_lean(self)),
        }
    }

    fn build_ln_def_from_vd_environment(
        &mut self,
        stmts: VdMirStmtIdxRange,
        environment_path: VdEnvironmentPath,
        module_path: VdModulePath,
    ) -> LnItemDefnData {
        // ad hoc
        let ident = self.mangle_hypothesis();
        let parameters = stmts
            .into_iter()
            .map_while(|stmt| self.build_ln_parameter_from_vd_stmt(stmt))
            .collect();
        LnItemDefnData::Def {
            ident,
            parameters,
            ty: None,
            body: LnMirDefBody::Tactics(stmts.to_lean(self)),
        }
    }

    fn build_ln_parameter_from_vd_stmt(&mut self, stmt: VdMirStmtIdx) -> Option<LnDefParameter> {
        let stmt_arena = self.stmt_arena();
        let stmt = &stmt_arena[stmt];
        match stmt {
            VdMirStmtData::LetPlaceholder { ref pattern, ty } => Some(LnDefParameter {
                ident: match *pattern {
                    VdMirPattern::Letter {
                        letter,
                        symbol_local_defn,
                    } => self.mangle_symbol(symbol_local_defn),
                    VdMirPattern::Assumed => todo!(),
                },
                ty: ty.to_lean(self),
            }),
            _ => None,
        }
    }
}

/// the rust documentation doesn't explain very clearly how `map_while` works
#[test]
fn map_while_documentation_works() {
    #[track_caller]
    fn t(a: &[bool], expected: usize) {
        let len = a.iter().copied().map_while(|a| a.then_some(())).count();
        assert_eq!(expected, len);
    }
    t(&[true, false, true], 1);
    t(&[true, true, true], 3);
}
