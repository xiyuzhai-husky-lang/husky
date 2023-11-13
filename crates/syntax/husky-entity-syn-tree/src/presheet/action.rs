use super::*;
use crate::ParentUseExpr;
use husky_entity_kind::TypeKind;
use husky_token::PathNameToken;

#[derive(Debug)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub(crate) enum PresheetAction {
    ResolveUseExpr {
        module_path: ModulePath,
        rule_idx: OnceUseRuleIdx,
        path_name_token: PathNameToken,
        symbol: EntitySymbol,
    },
    UpdateUseAllFromModuleRule {
        module_path: ModulePath,
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
            | PresheetAction::UpdateUseAllFromModuleRule { module_path, .. }
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
                let (path_name_token, symbol) = match rule.parent() {
                    Some(parent) => match rule.variant() {
                        OnceUseRuleVariant::Leaf { ident_token }
                        | OnceUseRuleVariant::Parent {
                            parent_name_token: PathNameToken::Ident(ident_token),
                            ..
                        } => (
                            (*ident_token).into(),
                            ctx.resolve_subitem(parent, ident_token.ident())
                                .ok_or(*ident_token),
                        ),
                        OnceUseRuleVariant::Parent {
                            parent_name_token: PathNameToken::SelfMod(_self_mod_token),
                            children: _,
                        } => {
                            todo!()
                        }
                        OnceUseRuleVariant::Parent {
                            parent_name_token: PathNameToken::Super(_),
                            children: _,
                        } => {
                            todo!()
                        }
                        OnceUseRuleVariant::Parent {
                            parent_name_token: PathNameToken::CrateRoot(_crate_token),
                            children: _,
                        } => {
                            // todo: prevent this in the parsing stage
                            todo!()
                        }
                        OnceUseRuleVariant::UseAllTypeVariants { parent_ty_path } => {
                            actions.push(PresheetAction::UseAllTypeVariants {
                                module_path: self.module_path,
                                rule_idx,
                                parent_ty_path: *parent_ty_path,
                            });
                            continue;
                        }
                    },
                    None => match rule.variant() {
                        OnceUseRuleVariant::Leaf { ident_token }
                        | OnceUseRuleVariant::Parent {
                            parent_name_token: PathNameToken::Ident(ident_token),
                            ..
                        } => {
                            let ident_token = *ident_token;
                            (
                                ident_token.into(),
                                ctx.resolve_root_ident(ident_token).ok_or(ident_token),
                            )
                        }
                        OnceUseRuleVariant::Parent {
                            parent_name_token: PathNameToken::SelfMod(self_value_token),
                            children: _,
                        } => (
                            (*self_value_token).into(),
                            Ok(EntitySymbol::SelfModule {
                                module_path: self.module_path,
                            }),
                        ),
                        OnceUseRuleVariant::Parent {
                            parent_name_token: PathNameToken::Super(super_token),
                            children: _,
                        } => match self.module_path.parent(ctx.db()) {
                            Some(super_module_path) => (
                                (*super_token).into(),
                                Ok(EntitySymbol::SuperModule {
                                    current_module_path: self.module_path,
                                    super_module_path,
                                }),
                            ),
                            None => todo!(),
                        },
                        OnceUseRuleVariant::Parent {
                            parent_name_token: PathNameToken::CrateRoot(crate_token),
                            children: _,
                        } => (
                            (*crate_token).into(),
                            Ok(EntitySymbol::CrateRoot {
                                root_module_path: ctx.crate_root(),
                            }),
                        ),
                        OnceUseRuleVariant::UseAllTypeVariants { parent_ty_path: _ } => todo!(),
                    },
                };
                actions.push(match symbol {
                    Ok(symbol) => PresheetAction::ResolveUseExpr {
                        module_path: self.module_path,
                        rule_idx,
                        symbol,
                        path_name_token,
                    },
                    Err(ident_token) => PresheetAction::Err {
                        module_path: self.module_path,
                        rule_idx,
                        error: OriginalEntityTreeError::UnresolvedRootIdent(ident_token).into(),
                    },
                })
            }
        }
        for (rule_idx, rule) in self.all_module_items_use_rules.indexed_iter() {
            if rule.is_unresolved(&ctx) {
                actions.push(PresheetAction::UpdateUseAllFromModuleRule {
                    module_path: self.module_path,
                    rule_idx,
                })
            }
        }
    }

    pub(crate) fn resolve_use_expr(
        &mut self,
        db: &dyn EntitySynTreeDb,
        rule_idx: OnceUseRuleIdx,
        _name_token: PathNameToken,
        original_symbol: EntitySymbol,
    ) {
        let rule = &mut self.once_use_rules[rule_idx];
        #[cfg(test)]
        assert!(rule.is_unresolved());
        rule.mark_as_resolved(original_symbol);
        let path = original_symbol.path(db);
        match rule.variant() {
            OnceUseRuleVariant::Parent {
                parent_name_token: _,
                children,
            } => {
                for child_use_expr_idx in children {
                    let use_expr = &self.use_expr_arena[child_use_expr_idx];
                    let rule = &self.once_use_rules[rule_idx];
                    match use_expr {
                        UseExpr::All { star_token: _ } => match path {
                            PrincipalEntityPath::Module(parent_path) => self
                                .all_module_items_use_rules
                                .push(UseAllModuleSymbolsRule::new(
                                    db,
                                    self,
                                    parent_path,
                                    rule.ast_idx(),
                                    child_use_expr_idx,
                                    rule.visibility(),
                                )),
                            PrincipalEntityPath::MajorItem(parent_module_item_path) => {
                                match parent_module_item_path {
                                    MajorItemPath::Type(parent_ty_path) => {
                                        match parent_ty_path.ty_kind(db) {
                                            TypeKind::Enum | TypeKind::Inductive => {
                                                self.once_use_rules.push(rule.new_nonroot(
                                                    child_use_expr_idx,
                                                    parent_ty_path.into(),
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
                                    OnceUseRuleVariant::Leaf {
                                        ident_token: *ident_token,
                                    },
                                );
                                self.once_use_rules.push(new_rule)
                            }
                            None => todo!(),
                        },
                        UseExpr::Parent(ParentUseExpr {
                            parent_name_token,
                            colon_colon_token: _,
                            children: Ok(children),
                        }) => match path.major() {
                            Some(path) => {
                                let new_rule = rule.new_nonroot(
                                    child_use_expr_idx,
                                    path,
                                    OnceUseRuleVariant::Parent {
                                        parent_name_token: *parent_name_token,
                                        children: children.idx_range(),
                                    },
                                );
                                self.once_use_rules.push(new_rule)
                            }
                            None => todo!(),
                        },
                        UseExpr::Parent(ParentUseExpr {
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
