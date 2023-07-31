use crate::*;
use salsa::DbWithJar;

pub trait SemanticTokenDb: DbWithJar<SemanticTokenJar> + TokenInfoDb {
    fn semantic_tokens_ext(
        &self,
        module_path: ModulePath,
        range: Option<TextRange>,
    ) -> EntitySynTreeResult<&[ext::SemanticToken]>;
}

impl<Db> SemanticTokenDb for Db
where
    Db: DbWithJar<SemanticTokenJar> + TokenInfoDb,
{
    fn semantic_tokens_ext(
        &self,
        module_path: ModulePath,
        _range: Option<TextRange>,
    ) -> EntitySynTreeResult<&[ext::SemanticToken]> {
        Ok(semantic_tokens_ext_without_range(self, module_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }
}
