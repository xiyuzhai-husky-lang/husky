use std::ops::ControlFlow;

use super::*;
use crate::ParentUseExprData;
use husky_entity_kind::TypeKind;
use husky_token::PathNameToken;

#[derive(Debug)]
#[salsa::debug_with_db]
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
        error: EntitySynTreeError,
    },
}

impl PresheetAction {
    pub(crate) fn module_path(&self) -> ModulePath {
        match self {
            PresheetAction::ResolveUseExpr { module_path, .. }
            | PresheetAction::UpdateUseAllRule {
                rule_module_path: module_path,
                ..
            }
            | PresheetAction::Err { module_path, .. } => *module_path,
            PresheetAction::UseAllTypeVariants { .. } => todo!(),
        }
    }
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
                OnceUseRuleVariant::Leaf { ident_token }
                | OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(ident_token),
                    ..
                } => (
                    ident_token.into(),
                    ctx.resolve_subitem_symbol(parent_major_entity_path, ident_token.ident())
                        .ok_or(t(ident_token)),
                ),
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
                OnceUseRuleVariant::Leaf { ident_token }
                | OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(ident_token),
                    ..
                } => (
                    ident_token.into(),
                    ctx.resolve_root_ident(ident_token).ok_or(t(ident_token)),
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

    pub(crate) fn resolve_use_expr(
        &mut self,
        db: &::salsa::Db,
        rule_idx: OnceUseRuleIdx,
        _name_token: PathNameToken,
        original_symbol: EntitySymbol,
    ) {
        let rule = &mut self.once_use_rules[rule_idx];
        #[cfg(test)]
        assert!(rule.is_unresolved());
        rule.mark_as_resolved(original_symbol);
        let path = original_symbol.principal_entity_path(db);
        match rule.variant() {
            OnceUseRuleVariant::Parent { children, .. } => {
                for child_use_expr_idx in children {
                    let use_expr = &self.use_expr_arena[child_use_expr_idx];
                    let rule = &self.once_use_rules[rule_idx];
                    match use_expr {
                        UseExpr::All { star_token: _ } => match path {
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
                                                    original_symbol,
                                                    OnceUseRuleVariant::UseAllTypeVariants {
                                                        parent_ty_path,
                                                    },
                                                ))
                                            }
                                            _ => todo!(),
                                        }
                                    }
                                    MajorItemPath::Trait(_) => todo!(),
                                    MajorItemPath::Fugitive(_) => todo!(),
                                }
                            }
                            PrincipalEntityPath::TypeVariant(_) => todo!(),
                        },
                        UseExpr::Leaf { ident_token } => match path.major() {
                            Some(path) => {
                                let new_rule = rule.new_nonroot(
                                    child_use_expr_idx,
                                    path,
                                    original_symbol,
                                    OnceUseRuleVariant::Leaf {
                                        ident_token: *ident_token,
                                    },
                                );
                                self.once_use_rules.push(new_rule)
                            }
                            None => todo!(),
                        },
                        UseExpr::Parent(ParentUseExprData {
                            parent_name_token,
                            children: Ok(children),
                            ..
                        }) => match path.major() {
                            Some(path) => {
                                let new_rule = rule.new_nonroot(
                                    child_use_expr_idx,
                                    path,
                                    original_symbol,
                                    OnceUseRuleVariant::Parent {
                                        parent_name_token: *parent_name_token,
                                        children: children.idx_range(),
                                    },
                                );
                                self.once_use_rules.push(new_rule)
                            }
                            None => todo!(),
                        },
                        UseExpr::Parent(ParentUseExprData {
                            children: Err(_), ..
                        })
                        | UseExpr::Err(_) => (),
                        UseExpr::SelfOne { self_mod_token: _ } => todo!(),
                    }
                }
            }
            OnceUseRuleVariant::Leaf { ident_token: _ } => {
                match self
                    .symbol_table
                    .push(EntitySymbolEntry::new_use_symbol_entry(
                        db,
                        original_symbol,
                        rule,
                    )) {
                    Ok(_) => (),
                    Err(_) => todo!(),
                }
            }
            OnceUseRuleVariant::UseAllTypeVariants { parent_ty_path: _ } => todo!(),
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
            Err(_) => todo!(),
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
            Err(_) => todo!(),
        }
    }

    pub(crate) fn mark_once_use_rule_as_erroneous(
        &mut self,
        rule_idx: OnceUseRuleIdx,
        error: EntitySynTreeError,
    ) {
        let rule = &mut self.once_use_rules[rule_idx];
        self.errors.push(error);
        rule.mark_as_erroneous()
    }

    pub(crate) fn mark_use_all_rule_as_erroneous(
        &mut self,
        rule_idx: UseAllModuleSymbolsRuleIdx,
        error: EntitySynTreeError,
    ) {
        let rule = &mut self.all_module_items_use_rules[rule_idx];
        self.errors.push(error);
        rule.mark_as_erroneous()
    }
}
