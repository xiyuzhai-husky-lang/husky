use crate::*;
use husky_accessibility::Accessibility;
use husky_entity_card::EntityCard;
use husky_entity_path::EntityPathData;
use husky_print_utils::p;
use husky_word::Identifier;
use idx_arena::ArenaIdx;
use salsa::DebugWithDb;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityNode {
    entity_path: EntityPath,
    accessibility: Accessibility,
    card: EntityCard,
}

impl std::ops::Deref for EntityTree {
    type Target = EntityNode;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl EntityNode {
    pub(crate) fn new(
        entity_path: EntityPath,
        accessibility: Accessibility,
        card: EntityCard,
    ) -> Self {
        Self {
            entity_path,
            accessibility,
            card,
        }
    }

    pub(crate) fn entity_path(&self) -> EntityPath {
        self.entity_path
    }

    pub(crate) fn accessibility(&self) -> Accessibility {
        self.accessibility
    }

    pub(crate) fn card(&self) -> EntityCard {
        self.card
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_node(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<EntityNode> {
    Ok(
        if let Some(parent_module) = parent_module(db, entity_path).as_ref()? {
            let entity_tree_sheet = db.primal_entity_tree_sheet(*parent_module).as_ref()?;
            if let Some(tree) = entity_tree_sheet.get(entity_path) {
                tree.node.clone()
            } else {
                p!(entity_path.show(db,));
                p!(entity_tree_sheet.show(db));
                todo!()
            }
        } else {
            let entity_tree_sheet = db.primal_entity_tree_sheet(entity_path).as_ref()?;
            EntityNode {
                entity_path,
                accessibility: Accessibility::Public,
                card: EntityCard::Module,
            }
        },
    )
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_card(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<EntityCard> {
    Ok(entity_node(db, entity_path).as_ref()?.card())
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_accessibility(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<Accessibility> {
    Ok(entity_node(db, entity_path).as_ref()?.accessibility())
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn parent_module(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<Option<EntityPath>> {
    Ok(match entity_path.data(db) {
        EntityPathData::CrateRoot(_) => None,
        EntityPathData::Childpath { parent, ident } => match entity_card(db, parent).as_ref()? {
            EntityCard::Module => Some(parent),
            _ => *parent_module(db, parent).as_ref()?,
        },
    })
}
