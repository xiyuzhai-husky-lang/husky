use crate::*;
use salsa::DbWithJar;

pub trait SemanticTokenDb: DbWithJar<SemanticTokenJar> + TokenInferDb {
    fn semantic_tokens_ext(
        &self,
        module_path: ModulePath,
    ) -> EntityTreeResult<&[ext::SemanticToken]>;
}

impl<Db> SemanticTokenDb for Db
where
    Db: DbWithJar<SemanticTokenJar> + TokenInferDb,
{
    fn semantic_tokens_ext(
        &self,
        module_path: ModulePath,
    ) -> EntityTreeResult<&[ext::SemanticToken]> {
        Ok(semantic_tokens_ext(self, module_path).as_ref()?)
    }
}
