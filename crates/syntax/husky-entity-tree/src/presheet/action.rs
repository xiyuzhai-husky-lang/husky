use super::*;
use with_db::{PartialOrdWithDb, WithDb};

#[derive(Debug)]
pub(crate) enum PresheetAction {
    ResolveUseExpr {
        module_path: ModulePath,
        rule_idx: UseTreeRuleIdx,
        symbol: EntitySymbol,
    },
    UpdateUseAll {
        module_path: ModulePath,
        rule_idx: UseAllRuleIdx,
    },
    Err {
        module_path: ModulePath,
        rule_idx: UseTreeRuleIdx,
        error: EntityTreeError,
    },
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for PresheetAction {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        match self {
            PresheetAction::ResolveUseExpr {
                module_path,
                rule_idx,
                symbol,
            } => f
                .debug_struct("ResolvedUseExpr")
                .field("module_path", &module_path.debug(db))
                .field("rule_idx", &rule_idx)
                .field("symbol", &symbol.debug(db))
                .finish(),
            PresheetAction::UpdateUseAll {
                module_path,
                rule_idx,
            } => f
                .debug_struct("UpdateUseAll")
                .field("module_path", &module_path.debug(db))
                .field("rule_idx", &rule_idx)
                .finish(),
            PresheetAction::Err { .. } => todo!(),
        }
    }
}

impl PresheetAction {
    pub(crate) fn module_path(&self) -> ModulePath {
        match self {
            PresheetAction::ResolveUseExpr { module_path, .. }
            | PresheetAction::UpdateUseAll { module_path, .. }
            | PresheetAction::Err { module_path, .. } => *module_path,
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
                actions.push(match symbol {
                    Some(symbol) => PresheetAction::ResolveUseExpr {
                        module_path: self.module_path,
                        rule_idx,
                        symbol,
                    },
                    None => PresheetAction::Err {
                        module_path: self.module_path,
                        rule_idx,
                        error: EntityTreeError::UnresolvedIdentifier(ident_token),
                    },
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
        rule_idx: UseTreeRuleIdx,
        symbol: EntitySymbol,
    ) {
        let rule = &mut self.use_expr_rules[rule_idx];
        #[cfg(test)]
        assert!(rule.is_unresolved());
        rule.mark_as_resolved();
        if !symbol.is_accessible_from(db, self.module_path) {
            self.errors.push(EntityTreeError::SymbolNotAccessible);
        }
        let path = symbol.path(db);
        if let Some(children) = rule.children() {
            for use_expr_idx in children {
                let use_expr = &self.use_expr_arena[use_expr_idx];
                let rule = &self.use_expr_rules[rule_idx];
                match use_expr {
                    UseExpr::All { star_token } => match path {
                        EntityPath::Module(path) => {
                            let new_rule = UseAllRule::new(
                                path,
                                rule.ast_idx(),
                                use_expr_idx,
                                rule.accessibility(),
                            );
                            self.use_all_rules.push(new_rule)
                        }
                        EntityPath::ModuleItem(_) => todo!(),
                        EntityPath::AssociatedItem(_) => todo!(),
                        EntityPath::Variant(_) => todo!(),
                    },
                    UseExpr::One { ident_token } => {
                        let new_rule = rule.new_child(*ident_token, use_expr_idx, path, None);
                        self.use_expr_rules.push(new_rule)
                    }
                    UseExpr::Parent {
                        ident_token,
                        scope_resolution_token,
                        children,
                    } => {
                        let new_rule = rule.new_child(
                            *ident_token,
                            use_expr_idx,
                            path,
                            Some(children.idx_range()),
                        );
                        self.use_expr_rules.push(new_rule)
                    }
                    UseExpr::Err(_) => todo!(),
                }
            }
        } else {
            match self
                .symbols
                .insert(EntitySymbolEntry::new_use_symbol_entry(db, symbol, rule))
            {
                Ok(_) => (),
                Err(_) => todo!(),
            }
        }
    }

    pub(crate) fn update_use_all(
        &mut self,
        rule_idx: UseAllRuleIdx,
        new_uses: Vec<EntitySymbolEntry>,
        progress: usize,
    ) {
        let rule = &mut self.use_all_rules[rule_idx];
        rule.set_progress(progress);
        match self.symbols.extend(new_uses) {
            Ok(_) => (),
            Err(_) => todo!(),
        }
    }

    pub(crate) fn mark_as_erroneous(&mut self, rule_idx: UseTreeRuleIdx, error: EntityTreeError) {
        let rule = &mut self.use_expr_rules[rule_idx];
        self.errors.push(error);
        rule.mark_as_erroneous()
    }
}
