use crate::*;
use salsa::DbWithJar;

pub trait SemanticTokenDb: DbWithJar<SemanticTokenJar> + TokenInfoDb {
    fn semantic_tokens_ext(
        &self,
        module_path: ModulePath,
    ) -> EntityTreeResult<&[ext::SemanticToken]>;
}

impl<Db> SemanticTokenDb for Db
where
    Db: DbWithJar<SemanticTokenJar> + TokenInfoDb,
{
    fn semantic_tokens_ext(
        &self,
        module_path: ModulePath,
    ) -> EntityTreeResult<&[ext::SemanticToken]> {
        Ok(semantic_tokens_ext(self, module_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }
}
