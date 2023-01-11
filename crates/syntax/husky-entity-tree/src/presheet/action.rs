use super::*;
use with_db::{PartialOrdWithDb, WithDb};

pub(crate) enum PresheetAction {
    ResolveEntityUse {
        module_path: ModulePath,
        entity_use_tracker_idx: UseOneTrackerIdx,
        entity_use_accessibility: Accessibility,
        symbol: EntitySymbol,
    },
    EvaluateUseAll {
        module_path: ModulePath,
        use_all_tracker_idx: UseAllTrackerIdx,
    },
}

impl PresheetAction {
    pub(crate) fn module_path(&self) -> ModulePath {
        match self {
            PresheetAction::ResolveEntityUse { module_path, .. }
            | PresheetAction::EvaluateUseAll { module_path, .. } => *module_path,
        }
    }
}

impl EntityTreePresheet {
    pub(crate) fn collect_possible_actions(
        &self,
        ctx: EntitySymbolContext,
        actions: &mut Vec<PresheetAction>,
    ) {
        for (entity_use_tracker_idx, entity_use_tracker) in self.use_one_trackers.indexed_iter() {
            if entity_use_tracker.is_unresolved() {
                let ident = entity_use_tracker.ident();
                if let Some(node) = ctx.resolve_ident(ident) {
                    actions.push(PresheetAction::ResolveEntityUse {
                        module_path: self.module_path,
                        entity_use_tracker_idx,
                        entity_use_accessibility: entity_use_tracker.accessibility(),
                        symbol: node.clone(),
                    })
                }
            }
        }
        for (use_all_tracker_idx, use_all_tracker) in self.use_all_trackers.indexed_iter() {
            todo!()
        }
    }

    pub(crate) fn exec(&mut self, db: &dyn EntityTreeDb, action: PresheetAction) {
        match action {
            PresheetAction::ResolveEntityUse {
                entity_use_tracker_idx,
                symbol: node,
                entity_use_accessibility,
                ..
            } => self.resolve_use_one(db, entity_use_tracker_idx, entity_use_accessibility, node),
            PresheetAction::EvaluateUseAll {
                module_path,
                use_all_tracker_idx,
            } => todo!(),
        }
    }
    fn resolve_use_one(
        &mut self,
        db: &dyn EntityTreeDb,
        entity_use_tracker_idx: UseOneTrackerIdx,
        entity_use_accessibility: Accessibility,
        symbol: EntitySymbol,
    ) {
        let tracker = &mut self.use_one_trackers[entity_use_tracker_idx];
        #[cfg(test)]
        assert!(tracker.is_unresolved());
        if !symbol.is_accessible_from(db, self.module_path) {
            todo!()
        }
        if !(entity_use_accessibility.with_db(db) <= symbol.accessility().with_db(db)) {
            todo!()
        }
        match self
            .module_specific_symbols
            .insert_new(EntitySymbol::EntityUse {
                ident: tracker.ident(),
                accessibility: entity_use_accessibility,
                path: symbol.entity_path(),
                ast_idx: tracker.ast_idx(),
                use_expr_idx: tracker.use_expr_idx(),
            }) {
            Ok(_) => (),
            Err(e) => {
                if e.new.entity_path() != self.module_specific_symbols.data()[e.old].entity_path() {
                    todo!()
                }
            }
        };
        tracker.mark_as_resolved()
    }
}
