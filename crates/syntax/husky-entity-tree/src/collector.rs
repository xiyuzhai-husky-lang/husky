mod action;

use crate::*;
use husky_print_utils::p;
use vec_like::{AsVecMapEntry, VecMap};

pub(crate) struct EntityTreeCollector<'a> {
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    presheets: VecMap<EntreePresheetMut<'a>>,
    core_prelude_module: ModulePath,
    // can't use `crate_prelude` here because it might not be available
    opt_universal_prelude: Option<EntitySymbolTableRef<'a>>,
    crate_specific_prelude: EntitySymbolTableRef<'a>,
    major_path_expr_arena: MajorPathExprArena,
    impl_blocks: Vec<ImplBlock>,
}

impl<'a> EntityTreeCollector<'a> {
    pub(crate) fn new(
        db: &'a dyn EntityTreeDb,
        crate_path: CratePath,
    ) -> EntityTreeCrateBundleResult<Self> {
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
                entree_presheet(db, *module_path)
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
                    module_entity_tree(db, core_prelude_module)
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
            presheets,
            core_prelude_module,
            opt_universal_prelude: universal_prelude,
            crate_specific_prelude: crate_specific_prelude(db, crate_path)
                .as_ref()
                .map(|table| table.as_ref())
                .map_err(|e| e.clone())?,
            major_path_expr_arena: Default::default(),
            impl_blocks: Default::default(),
        })
    }

    pub(crate) fn collect_all(mut self) -> EntityTreeCrateBundle {
        loop {
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
        let impl_blockss = self.collect_impl_blockss();
        let sheets = std::iter::zip(self.presheets.into_iter(), impl_blockss.into_iter())
            .map(|(presheet, impl_blocks)| presheet.into_sheet(impl_blocks))
            .collect();
        EntityTreeCrateBundle::new(sheets, self.major_path_expr_arena, self.impl_blocks)
    }

    fn collect_impl_blockss(&mut self) -> Vec<Vec<ImplBlock>> {
        let mut impl_blockss = vec![];
        for presheet in self.presheets.iter() {
            let module_path = presheet.module_path();
            let ast_sheet = self.db.ast_sheet(module_path).unwrap();
            let impl_blocks = ast_sheet
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
            impl_blockss.push(impl_blocks);
        }
        impl_blockss
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
                let new_uses = self.presheets[parent].module_specific_symbols().data()[progress..]
                    .iter()
                    .filter_map(|entry| {
                        let symbol = entry.symbol();
                        symbol
                            .is_accessible_from(db, module_path)
                            .then_some(UseSymbol::new(
                                db,
                                symbol.path(db),
                                entry.ident(),
                                rule.accessibility(),
                                rule.ast_idx(),
                                rule.use_expr_idx(),
                            ))
                    })
                    .collect();
                self.presheets[module_path].update_use_all(rule_idx, new_uses)
            }
        }
    }
}

fn crate_prelude<'a>(
    opt_universal_prelude: Option<EntitySymbolTableRef<'a>>,
    core_prelude_module: ModulePath,
    presheets: &'a VecMap<EntreePresheetMut<'a>>,
    crate_specific_prelude: EntitySymbolTableRef<'a>,
) -> CrateSymbolContext<'a> {
    let universal_prelude = opt_universal_prelude
        .unwrap_or_else(|| presheets[core_prelude_module].module_specific_symbols());
    CrateSymbolContext::new(universal_prelude, crate_specific_prelude)
}
