use crate::*;
use husky_word::InternWord;
use std::sync::Arc;

#[salsa::query_group(EntityPathDbStorage)]
pub trait EntityPathDb: InternEntityPath + InternWord {
    fn entity_path_menu(&self) -> Arc<EntityPathMenu>;
}

fn entity_path_menu(db: &dyn EntityPathDb) -> Arc<EntityPathMenu> {
    Arc::new(EntityPathMenu::new(db))
}
