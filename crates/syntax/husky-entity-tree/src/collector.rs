mod action;

use crate::*;
use husky_print_utils::p;
use vec_like::VecMap;

pub(crate) struct EntityTreeCollector<'a> {
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    crate_root: ModulePath,
    impl_registry: ImplBlockRegistry,
    presheets: VecMap<EntityTreePresheetMut<'a>>,
    core_prelude_module: ModulePath,
    // can't use `crate_prelude` here because it might not be available
    opt_universal_prelude: Option<EntitySymbolTableRef<'a>>,
    crate_specific_prelude: EntitySymbolTableRef<'a>,
    major_path_expr_arena: MajorPathExprArena,
    impls: Vec<ImplBlock>,
}

impl<'a> EntityTreeCollector<'a> {
    pub(crate) fn new(
        db: &'a dyn EntityTreeDb,
        crate_path: CratePath,
    ) -> EntityTreeCrateBundleResult<Self> {
        let crate_root = ModulePath::new_root(db, crate_path);
        let all_modules = db.all_modules_within_crate(crate_path);
        for i in 0..all_modules.len() {
            for j in (i + 1)..all_modules.len() {
                if all_modules[i] == all_modules[j] {
                    p!(all_modules[i].debug(db))
                }
                assert!(all_modules[i] != all_modules[j])
            }
        }
        let presheets = all_modules
            .into_iter()
            .filter_map(|module_path| {
                entity_tree_presheet(db, *module_path)
                    .as_ref()
                    .ok()
                    .map(|presheet| presheet.presheet_mut(db))
            })
            .collect();
        let toolchain = crate_path.toolchain(db);
        let path_menu = db.vfs_path_menu(toolchain)?;
        let core_prelude_module = path_menu.core_prelude();
        let universal_prelude: Option<EntitySymbolTableRef<'a>> = {
            if crate_path != path_menu.core_library() {
                Some(
                    entity_tree_sheet(db, core_prelude_module)
                        .map_err(|e| PreludeError::CorePreludeEntityTreeSheet(Box::new(e)))?
                        .module_symbols(),
                )
            } else {
                None
            }
        };
        Ok(Self {
            db,
            crate_path,
            crate_root,
            impl_registry: ImplBlockRegistry::default(),
            presheets,
            core_prelude_module,
            opt_universal_prelude: universal_prelude,
            crate_specific_prelude: crate_specific_prelude(db, crate_path)
                .as_ref()
                .map(|table| table.as_ref())
                .map_err(|e| e.clone())?,
            major_path_expr_arena: Default::default(),
            impls: Default::default(),
        })
    }

    pub(crate) fn collect_all(mut self) -> EntityTreeCrateBundle {
        // for testing purposes
        let mut loop_idx = 0;
        const LOOP_LIMIT: usize = 100;
        loop {
            loop_idx += 1;
            if loop_idx >= LOOP_LIMIT {
                panic!();
            }
            let actions = self.collect_possible_actions();
            if actions.len() == 0 {
                break;
            }
            for action in actions {
                self.exec(action)
            }
        }
        #[cfg(test)]
        for presheet in self.presheets.iter() {
            presheet.check_done(self.db)
        }
        let implss = self.collect_implss();
        let sheets = std::iter::zip(self.presheets.into_iter(), implss.into_iter())
            .map(|(presheet, impls)| presheet.into_sheet(impls))
            .collect();
        EntityTreeCrateBundle::new(sheets, self.major_path_expr_arena, self.impls)
    }

    fn collect_implss(&mut self) -> Vec<Vec<ImplBlock>> {
        let mut implss = vec![];
        for presheet in self.presheets.iter() {
            let module_path = presheet.module_path();
            let ast_sheet = self.db.ast_sheet(module_path).unwrap();
            let impls = ast_sheet
                .all_ast_indexed_iter()
                .filter_map(|(ast_idx, ast)| match ast {
                    Ast::Impl {
                        token_group_idx,
                        body,
                    } => {
                        let crate_prelude = crate_prelude(
                            self.opt_universal_prelude,
                            self.core_prelude_module,
                            &self.presheets,
                            self.crate_specific_prelude,
                        );
                        let module_symbol_context = ModuleSymbolContext::new(
                            crate_prelude,
                            presheet.module_specific_symbols(),
                        );
                        Some(ImplBlock::parse_from_token_group(
                            self.db,
                            &mut self.impl_registry,
                            module_symbol_context,
                            module_path,
                            ast_idx,
                            *body,
                            self.db
                                .token_sheet_data(module_path)
                                .unwrap()
                                .token_group_token_stream(*token_group_idx, None),
                            &mut self.major_path_expr_arena,
                        ))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            implss.push(impls);
        }
        implss
    }

    fn exec(&mut self, action: PresheetAction) {
        let db = self.db;
        match action {
            PresheetAction::ResolveUseExpr {
                module_path,
                rule_idx,
                symbol: export,
            } => self.presheets[module_path].resolve_use_expr(db, rule_idx, export),
            PresheetAction::UpdateUseAll {
                module_path,
                rule_idx,
            } => {
                let rule = &self.presheets[module_path][rule_idx];
                let parent = rule.parent();
                let progress = rule.progress();
                let parent_symbols = &self.presheets[parent].module_specific_symbols().data();
                let new_uses: Vec<EntitySymbolEntry> = parent_symbols[progress..]
                    .iter()
                    .filter_map(|entry| entry.export_via_use_all(db, module_path, rule))
                    .collect();
                let progress = parent_symbols.len();
                self.presheets[module_path].update_use_all(rule_idx, new_uses, progress)
            }
            PresheetAction::Err {
                module_path,
                rule_idx,
                error,
            } => self.presheets[module_path].mark_as_erroneous(rule_idx, error),
        }
    }
}

fn crate_prelude<'a>(
    opt_universal_prelude: Option<EntitySymbolTableRef<'a>>,
    core_prelude_module: ModulePath,
    presheets: &'a VecMap<EntityTreePresheetMut<'a>>,
    crate_specific_prelude: EntitySymbolTableRef<'a>,
) -> CrateSymbolContext<'a> {
    let universal_prelude = opt_universal_prelude
        .unwrap_or_else(|| presheets[core_prelude_module].module_specific_symbols());
    CrateSymbolContext::new(universal_prelude, crate_specific_prelude)
}
