use super::*;
use crate::ParentUseExprData;
use husky_entity_kind::TypeKind;
use husky_token::PathNameToken;

#[derive(Debug)]
#[salsa::derive_debug_with_db]
pub(crate) enum PresheetAction {
    ResolveUseExpr {
        module_path: ModulePath,
        rule_idx: OnceUseRuleIdx,
        path_name_token: PathNameToken,
        symbol: EntitySymbol,
    },
    UpdateUseAllRule {
        /// this is the module path where the rule appears,
        /// not the source to use all from
        rule_module_path: ModulePath,
        rule_idx: UseAllModuleSymbolsRuleIdx,
    },
    UseAllTypeVariants {
        module_path: ModulePath,
        rule_idx: OnceUseRuleIdx,
        parent_ty_path: TypePath,
    },
    Err {
        module_path: ModulePath,
        rule_idx: OnceUseRuleIdx,
        error: EntityTreeError,
    },
}

impl<'a> EntityTreePresheetMut<'a> {
    pub(crate) fn collect_possible_actions(
        &self,
        ctx: EntityTreeSymbolContext<'a, '_>,
        actions: &mut Vec<PresheetAction>,
    ) {
        for (rule_idx, rule) in self.once_use_rules.indexed_iter() {
            if rule.is_unresolved() {
                self.collect_possible_actions_from_once_use_rule(rule_idx, rule, &ctx, actions)
            }
        }
        for (rule_idx, rule) in self.all_module_items_use_rules.indexed_iter() {
            if rule.is_actionable(&ctx) {
                actions.push(PresheetAction::UpdateUseAllRule {
                    rule_module_path: self.module_path,
                    rule_idx,
                })
            }
        }
    }

    fn collect_possible_actions_from_once_use_rule(
        &self,
        rule_idx: OnceUseRuleIdx,
        rule: &OnceUseRule,
        ctx: &EntityTreeSymbolContext<'_, '_>,
        actions: &mut Vec<PresheetAction>,
    ) {
        let db = ctx.db();
        if let OnceUseRuleVariant::UseAllTypeVariants { parent_ty_path } = *rule.variant() {
            actions.push(PresheetAction::UseAllTypeVariants {
                module_path: self.module_path,
                rule_idx,
                parent_ty_path,
            });
            return;
        }
        let t = |ident_token| OriginalEntityTreeError::UnresolvedRootIdent(ident_token).into();
        let (path_name_token, symbol_result) = match rule.parent() {
            Some((parent_major_entity_path, parent_original_symbol)) => match *rule.variant() {
                OnceUseRuleVariant::IdentLeaf { ident_token }
                | OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(ident_token),
                    ..
                } => (
                    ident_token.into(),
                    ctx.resolve_subitem_symbol(parent_major_entity_path, ident_token.ident())
                        .ok_or(t(ident_token)),
                ),
                OnceUseRuleVariant::SelfLeaf { self_mod_token } => {
                    (self_mod_token.into(), Ok(parent_original_symbol))
                }
                OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::SelfMod(self_mod_token),
                    ..
                } => (self_mod_token.into(), Ok(parent_original_symbol)),
                OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Super(super_mod_token),
                    ..
                } => (
                    super_mod_token.into(),
                    parent_original_symbol.super_symbol(db),
                ),
                OnceUseRuleVariant::Parent { .. } => {
                    unreachable!("should be prevented in the parsing stage")
                }
                OnceUseRuleVariant::UseAllTypeVariants { .. } => {
                    unreachable!("already handled")
                }
            },
            None => match *rule.variant() {
                OnceUseRuleVariant::IdentLeaf { ident_token }
                | OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(ident_token),
                    ..
                } => (
                    ident_token.into(),
                    ctx.resolve_root_ident(ident_token).ok_or(t(ident_token)),
                ),
                OnceUseRuleVariant::SelfLeaf { self_mod_token } => (
                    self_mod_token.into(),
                    Err(OriginalEntityTreeError::StandaloneSelf.into()),
                ),
                OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::SelfMod(self_value_token),
                    ..
                } => (
                    self_value_token.into(),
                    Ok(EntitySymbol::RootSelfModule {
                        module_path: self.module_path,
                    }),
                ),
                OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Super(super_token),
                    ..
                } => {
                    let symbol_result = match self.module_path.parent(ctx.db()) {
                        Some(super_module_path) => Ok(EntitySymbol::RootSuperModule {
                            current_module_path: self.module_path,
                            super_module_path,
                        }),
                        None => {
                            Err(OriginalEntityTreeError::NoSuperForCrateRoot { super_token }.into())
                        }
                    };
                    (super_token.into(), symbol_result)
                }
                OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(crate_token),
                    ..
                } => (
                    crate_token.into(),
                    Ok(EntitySymbol::CrateRoot {
                        root_module_path: ctx.crate_root(),
                    }),
                ),
                OnceUseRuleVariant::UseAllTypeVariants { .. } => {
                    unreachable!("already handled")
                }
            },
        };
        actions.push(match symbol_result {
            Ok(symbol) => PresheetAction::ResolveUseExpr {
                module_path: self.module_path,
                rule_idx,
                symbol,
                path_name_token,
            },
            Err(error) => PresheetAction::Err {
                module_path: self.module_path,
                rule_idx,
                error,
            },
        });
    }

    /// two things:
    /// - mark the rule as resolved or erroneous given the original symbol
    /// - add rules generated by this rule if parent or add symbols if leaf
    pub(crate) fn resolve_use_expr_leading_symbol(
        &mut self,
        db: &::salsa::Db,
        rule_idx: OnceUseRuleIdx,
        name_token: PathNameToken,
        leading_symbol: EntitySymbol,
    ) {
        let path = leading_symbol.principal_entity_path(db);
        self.mark_rule(rule_idx, leading_symbol, path, db, name_token);
        self.add_generated_rules_or_add_symbol(rule_idx, leading_symbol, path, db);
    }

    fn mark_rule(
        &mut self,
        rule_idx: OnceUseRuleIdx,
        original_symbol: EntitySymbol,
        path: PrincipalEntityPath,
        _db: &salsa::Db,
        name_token: PathNameToken,
    ) {
        let rule = &mut self.once_use_rules[rule_idx];
        // mark the rule as resolved or erroneous given the original symbol
        #[cfg(test)]
        assert!(rule.is_unresolved());
        match (rule.variant(), path) {
            (
                OnceUseRuleVariant::Parent { .. },
                PrincipalEntityPath::TypeVariant(_)
                | PrincipalEntityPath::MajorItem(MajorItemPath::Trait(_) | MajorItemPath::Form(_)),
            ) => {
                self.errors.push(
                    OriginalEntityTreeError::InvalidParentPath {
                        name_token,
                        principal_entity_path: path,
                    }
                    .into(),
                );
                rule.mark_as_erroneous()
            }
            _ => rule.mark_as_resolved(original_symbol),
        }
    }

    /// add rules generated by this rule if parent or add symbols if leaf
    fn add_generated_rules_or_add_symbol(
        &mut self,
        rule_idx: OnceUseRuleIdx,
        leading_symbol: EntitySymbol,
        // precomputed to be equal to `original_symbol.principal_entity_path(db)`
        path: PrincipalEntityPath,
        db: &salsa::Db,
    ) {
        let rule = &mut self.once_use_rules[rule_idx];
        match *rule.variant() {
            OnceUseRuleVariant::Parent { children, .. } => {
                for child_use_expr_idx in children {
                    let use_expr = &self.use_expr_arena[child_use_expr_idx];
                    let rule = &self.once_use_rules[rule_idx];
                    match *use_expr {
                        UseExpr::All { .. } => match path {
                            PrincipalEntityPath::Module(parent_path) => {
                                self.all_module_items_use_rules.push(UseAllRule::new(
                                    db,
                                    self,
                                    parent_path,
                                    rule.ast_idx(),
                                    child_use_expr_idx,
                                    rule.visibility(),
                                ))
                            }
                            PrincipalEntityPath::MajorItem(parent_module_item_path) => {
                                match parent_module_item_path {
                                    MajorItemPath::Type(parent_ty_path) => {
                                        match parent_ty_path.ty_kind(db) {
                                            TypeKind::Enum | TypeKind::Inductive => {
                                                self.once_use_rules.push(rule.new_nonroot(
                                                    child_use_expr_idx,
                                                    parent_ty_path.into(),
                                                    leading_symbol,
                                                    OnceUseRuleVariant::UseAllTypeVariants {
                                                        parent_ty_path,
                                                    },
                                                ))
                                            }
                                            _ => (),
                                        }
                                    }
                                    MajorItemPath::Trait(_) | MajorItemPath::Form(_) => (),
                                }
                            }
                            PrincipalEntityPath::TypeVariant(_) => (),
                        },
                        UseExpr::IdentLeaf { ident_token } => {
                            if let Some(path) = path.major() {
                                let new_rule = rule.new_nonroot(
                                    child_use_expr_idx,
                                    path,
                                    leading_symbol,
                                    OnceUseRuleVariant::IdentLeaf { ident_token },
                                );
                                self.once_use_rules.push(new_rule)
                            }
                        }
                        UseExpr::SelfLeaf { self_mod_token } => {
                            if let Some(path) = path.major() {
                                let new_rule = rule.new_nonroot(
                                    child_use_expr_idx,
                                    path,
                                    leading_symbol,
                                    OnceUseRuleVariant::SelfLeaf { self_mod_token },
                                );
                                self.once_use_rules.push(new_rule)
                            }
                        }
                        UseExpr::Parent(ParentUseExprData {
                            parent_name_token,
                            children: Ok(ref children),
                            ..
                        }) => {
                            if let Some(path) = path.major() {
                                let new_rule = rule.new_nonroot(
                                    child_use_expr_idx,
                                    path,
                                    leading_symbol,
                                    OnceUseRuleVariant::Parent {
                                        parent_name_token,
                                        children: children.idx_range(),
                                    },
                                );
                                self.once_use_rules.push(new_rule)
                            }
                        }
                        UseExpr::Parent(ParentUseExprData {
                            children: Err(_), ..
                        })
                        | UseExpr::Err(_) => (),
                    }
                }
            }
            OnceUseRuleVariant::IdentLeaf { .. } | OnceUseRuleVariant::SelfLeaf { .. } => {
                match self
                    .symbol_table
                    .push(EntitySymbolEntry::new_use_symbol_entry(
                        db,
                        leading_symbol,
                        rule,
                    )) {
                    Ok(_) => (),
                    Err(error) => self.errors.push(error),
                }
            }
            OnceUseRuleVariant::UseAllTypeVariants { parent_ty_path: _ } => unreachable!("rule `UseAllTypeVariants` does not need to resolve leading symbol, so cannot reach here"),
        }
    }

    pub(crate) fn update_module_use_all_rule(
        &mut self,
        rule_idx: UseAllModuleSymbolsRuleIdx,
        new_uses: Vec<EntitySymbolEntry>,
        progress: usize,
    ) {
        self.all_module_items_use_rules[rule_idx].set_progress(progress);
        match self.symbol_table.extend(new_uses) {
            Ok(_) => (),
            Err(error) => self.errors.push(error),
        }
    }

    pub(crate) fn update_use_all_ty_variants_rule(
        &mut self,
        rule_idx: OnceUseRuleIdx,
        new_uses: Vec<EntitySymbolEntry>,
    ) {
        self.once_use_rules[rule_idx].mark_as_resolved(None);
        match self.symbol_table.extend(new_uses) {
            Ok(_) => (),
            Err(error) => self.errors.push(error),
        }
    }

    pub(crate) fn mark_once_use_rule_as_erroneous(
        &mut self,
        rule_idx: OnceUseRuleIdx,
        error: EntityTreeError,
    ) {
        let rule = &mut self.once_use_rules[rule_idx];
        self.errors.push(error);
        rule.mark_as_erroneous()
    }
}
