mod action;

use crate::*;
use husky_print_utils::p;
use vec_like::{AsVecMapEntry, VecMap};

pub(crate) struct EntityTreeCollector<'a> {
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    presheets: VecMap<EntityTreePresheet>,
    core_prelude_module: ModulePath,
    // can't use `crate_prelude` here because it might not be available
    opt_universal_prelude: Option<&'a [EntitySymbol]>,
    crate_specific_prelude: &'a [EntitySymbol],
    principal_entity_path_expr_arena: PrincipalPathExprArena,
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
            .filter_map(|module_path| entity_tree_presheet(db, *module_path).clone().ok())
            .collect();
        let toolchain = crate_path.toolchain(db);
        let path_menu = db.vfs_path_menu(toolchain)?;
        let core_prelude_module = path_menu.core_prelude();
        let universal_prelude: Option<&'a [EntitySymbol]> = {
            if crate_path != path_menu.core_library() {
                Some(
                    module_entity_tree(db, core_prelude_module)
                        .map_err(|e| PreludeError::CorePreludeEntityTreeSheet(Box::new(e)))?
                        .module_specific_symbols(),
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
                .map_err(|e| e.clone())?,
            principal_entity_path_expr_arena: Default::default(),
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
        let impl_blockss = self.collect_impl_blockss();
        let sheets = std::iter::zip(self.presheets.into_iter(), impl_blockss.into_iter())
            .map(|(presheet, impl_blocks)| presheet.into_sheet(impl_blocks))
            .collect();
        EntityTreeCrateBundle::new(
            sheets,
            self.principal_entity_path_expr_arena,
            self.impl_blocks,
        )
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
                            &mut self.principal_entity_path_expr_arena,
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
        let module_path = action.module_path();
        let presheet = &mut self.presheets[module_path];
        presheet.exec(self.db, action)
    }
}

fn crate_prelude<'a>(
    opt_universal_prelude: Option<&'a [EntitySymbol]>,
    core_prelude_module: ModulePath,
    presheets: &'a VecMap<EntityTreePresheet>,
    crate_specific_prelude: &'a [EntitySymbol],
) -> CrateSymbolContext<'a> {
    let universal_prelude = opt_universal_prelude
        .unwrap_or_else(|| presheets[core_prelude_module].module_specific_symbols());
    CrateSymbolContext::new(universal_prelude, crate_specific_prelude)
}
