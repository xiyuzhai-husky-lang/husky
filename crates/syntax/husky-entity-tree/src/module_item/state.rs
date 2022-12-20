use vec_like::{VecEntryMap, VecPairMap};

use super::*;

#[derive(Debug)]
pub(super) struct CollectorState<'a> {
    module_item_maps: VecPairMap<EntityPath, IdentMap<ModuleItem>>,
    unresolved_use_exprs: VecPairMap<EntityPath, UnresolvedEntityUseExprs<'a>>,
    use_alls: Vec<UseAll>,
    has_changed: bool,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct UseAll {
    accessibility: Accessibility,
    module: EntityPath,
    use_all_parent: EntityPath,
}

#[derive(Debug)]
pub(super) struct UnresolvedEntityUseExprs<'a> {
    pub(super) sheet: &'a EntityUseExprSheet,
    pub(super) exprs: Vec<UnresolvedUseExpr>,
}

#[derive(Debug)]
pub(super) struct UnresolvedUseExpr {
    pub(super) accessibility: Accessibility,
    pub(super) ident: Identifier,
    pub(super) use_expr_idx: EntityUseExprIdx,
}

impl<'a> CollectorState<'a> {
    pub(super) fn new(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> EntityTreeResult<Self> {
        let all_modules = all_modules_within_crate(db, crate_path).as_ref()?;
        Ok(Self {
            module_item_maps: all_modules
                .iter()
                .map(|module| -> EntityTreeResult<(_, _)> {
                    let entity_tree_sheet = db.entity_tree_sheet(*module).as_ref()?;
                    let module_defn_items: IdentMap<ModuleItem> = entity_tree_sheet
                        .top_level_entities()
                        .filter_map(|(tree_idx, accessibility, entity_card, entity_path)| {
                            match entity_card {
                                EntityCard::Module
                                | EntityCard::Type
                                | EntityCard::Trait
                                | EntityCard::Form => Some(ModuleItem::Defn {
                                    ident: match entity_path.data(db) {
                                        EntityPathData::CrateRoot(_) => unreachable!(),
                                        EntityPathData::Childpath { parent: _, ident } => ident,
                                    },
                                    accessibility,
                                    tree_idx,
                                }),
                                EntityCard::Use => None,
                                EntityCard::EnumVariant => unreachable!(),
                            }
                        })
                        .collect();
                    Ok((*module, module_defn_items))
                })
                .collect::<EntityTreeResult<VecEntryMap<_, _>>>()?,
            unresolved_use_exprs: all_modules
                .iter()
                .map(|module| -> EntityTreeResult<(_, _)> {
                    let sheet = module_use_exprs(db, *module).as_ref()?;
                    let unresolved_use_exprs: Vec<UnresolvedUseExpr> = sheet
                        .use_exprs()
                        .iter()
                        .map(
                            |(accessibility, ident, use_expr_idx, _ast_idx)| UnresolvedUseExpr {
                                accessibility: *accessibility,
                                ident: *ident,
                                use_expr_idx: *use_expr_idx,
                            },
                        )
                        .collect();
                    Ok((
                        *module,
                        UnresolvedEntityUseExprs {
                            sheet,
                            exprs: unresolved_use_exprs,
                        },
                    ))
                })
                .collect::<EntityTreeResult<VecEntryMap<_, _>>>()?,
            use_alls: vec![],
            has_changed: false,
        })
    }

    pub(super) fn finish(self, db: &dyn EntityTreeDb) -> VecPairMap<EntityPath, ModuleItemMap> {
        self.module_item_maps
            .into_iter()
            .map(|(entity_path, map)| (entity_path, ModuleItemMap::new(db, map)))
            .collect()
    }

    pub(super) fn has_changed(&self) -> bool {
        self.has_changed
    }

    pub(super) fn reset_change_flag(&mut self) {
        self.has_changed = false
    }

    pub(super) fn unresolved_use_exprs(&self) -> &[(EntityPath, UnresolvedEntityUseExprs<'a>)] {
        &self.unresolved_use_exprs
    }

    pub(super) fn use_alls(&self) -> &[UseAll] {
        self.use_alls.as_ref()
    }

    pub(super) fn module_item_maps(&self) -> &VecPairMap<EntityPath, IdentMap<ModuleItem>> {
        &self.module_item_maps
    }
}
