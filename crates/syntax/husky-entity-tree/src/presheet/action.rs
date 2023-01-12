use super::*;
use with_db::{PartialOrdWithDb, WithDb};

pub(crate) enum PresheetAction {
    ResolveUseExpr {
        module_path: ModulePath,
        tracker_idx: UseTreeRuleIdx,
        symbol: EntitySymbol,
    },
    UpdateUseAll {
        module_path: ModulePath,
        rule_idx: UseAllRuleIdx,
    },
}

impl PresheetAction {
    pub(crate) fn module_path(&self) -> ModulePath {
        match self {
            PresheetAction::ResolveUseExpr { module_path, .. }
            | PresheetAction::UpdateUseAll { module_path, .. } => *module_path,
        }
    }
}

impl<'a> EntreePresheetMut<'a> {
    pub(crate) fn collect_possible_actions(
        &self,
        ctx: EntreeSymbolContext,
        actions: &mut Vec<PresheetAction>,
    ) {
        for (rule_idx, rule) in self.use_expr_rules.indexed_iter() {
            if rule.is_unresolved() {
                let ident_token = rule.ident_token();
                let ident = ident_token.ident();
                let symbol = match rule.parent() {
                    Some(parent) => ctx.resolve_subentity(parent, ident),
                    None => ctx.resolve_ident(ident),
                };
                let Some(symbol) = symbol else {
                    todo!()
                };
                actions.push(PresheetAction::ResolveUseExpr {
                    module_path: self.module_path,
                    tracker_idx: rule_idx,
                    symbol,
                })
            }
        }
        for (rule_idx, rule) in self.use_all_rules.indexed_iter() {
            if rule.is_unresolved(&ctx) {
                actions.push(PresheetAction::UpdateUseAll {
                    module_path: self.module_path,
                    rule_idx,
                })
            }
        }
    }

    pub(crate) fn resolve_use_expr(
        &mut self,
        db: &dyn EntityTreeDb,
        use_tree_rule_idx: UseTreeRuleIdx,
        symbol: EntitySymbol,
    ) {
        let rule = &mut self.use_expr_rules[use_tree_rule_idx];
        #[cfg(test)]
        assert!(rule.is_unresolved());
        rule.mark_as_resolved();
        if !symbol.is_accessible_from(db, self.module_path) {
            todo!()
        }
        let path = symbol.path(db);
        for use_tree_expr_idx in rule.children() {
            let use_tree_expr = &self.use_tree_expr_arena[use_tree_expr_idx];
            match use_tree_expr {
                UseExpr::All { star_token } => match path {
                    EntityPath::Module(path) => {
                        let new_rule = UseAllRule::new(path);
                        self.use_all_rules.push(new_rule)
                    }
                    EntityPath::ModuleItem(_) => todo!(),
                    EntityPath::AssociatedItem(_) => todo!(),
                    EntityPath::Variant(_) => todo!(),
                },
                UseExpr::One { ident_token } => todo!(),
                UseExpr::Parent {
                    ident_token,
                    scope_resolution_token,
                    children,
                } => {
                    let new_rule = self.use_expr_rules[use_tree_rule_idx].new_child(
                        *ident_token,
                        use_tree_expr_idx,
                        path,
                        children.idx_range(),
                    );
                    self.use_expr_rules.push(new_rule)
                }
                UseExpr::Err(_) => todo!(),
            }
        }
        // if !(tracker.accessibility().with_db(db) <= symbol.accessility().with_db(db)) {
        //     p!(
        //         tracker.accessibility().debug(db),
        //         symbol.accessility().debug(db),
        //         symbol.debug(db)
        //     );
        //     todo!()
        // }
        // let use_tree_expr = &self.use_tree_expr_arena[tracker.use_tree_expr_idx()];
        // match use_tree_expr {
        //     UseExpr::All { start } => todo!(),
        //     UseExpr::One { ident } => todo!(),
        //     UseExpr::Parent {
        //         ident,
        //         scope_resolution_token,
        //         children,
        //     } => todo!(),
        //     UseExpr::Err(_) => todo!(),
        // }
        // match self
        //     .module_specific_symbols
        //     .insert_new(EntitySymbol::EntityUse {
        //         ident: tracker.ident(),
        //         accessibility: use_accessibility,
        //         path: symbol.entity_path(),
        //         ast_idx: tracker.ast_idx(),
        //         use_expr_idx: tracker.use_expr_idx(),
        //     }) {
        //     Ok(_) => (),
        //     Err(e) => {
        //         if e.new.entity_path() != self.module_specific_symbols.data()[e.old].entity_path() {
        //             todo!()
        //         }
        //     }
        // }
    }

    pub(crate) fn update_use_all(&mut self) {
        todo!()
    }
}
