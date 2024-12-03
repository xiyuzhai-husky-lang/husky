use crate::*;

#[salsa::jar]
pub struct BaseVfsJar(VfsCache, File);

impl salsa::storage::IngredientsFor for VfsCache {
    type Jar = BaseVfsJar;

    type Ingredients = Self;

    fn create_ingredients(_routes: &mut salsa::routes::Routes) -> Self::Ingredients {
        Default::default()
    }
}

impl BaseVfsJar {
    pub(crate) fn cache(&self) -> &VfsCache {
        &self.0
    }

    // pub(crate) fn set_watcher(&mut self, watcher: VfsWatcher) {
    //     self.0.set_watcher(watcher)
    // }
}
