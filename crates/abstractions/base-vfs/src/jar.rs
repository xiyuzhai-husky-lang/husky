use crate::*;

#[salsa::jar]
pub struct BaseVfsJar(BaseVfsCache, File);

impl salsa::storage::IngredientsFor for BaseVfsCache {
    type Jar = BaseVfsJar;

    type Ingredients = Self;

    fn create_ingredients(_routes: &mut salsa::routes::Routes) -> Self::Ingredients {
        Default::default()
    }
}

impl BaseVfsJar {
    pub(crate) fn cache(&self) -> &BaseVfsCache {
        &self.0
    }
}
