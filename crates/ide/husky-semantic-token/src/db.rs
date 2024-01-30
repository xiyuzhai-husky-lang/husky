use crate::*;

pub trait SemanticTokenDb {
    fn semantic_tokens_ext(
        &self,
        module_path: ModulePath,
        range: Option<TextRange>,
    ) -> EntityTreeResult<&[ext::SemanticToken]>;
}

impl SemanticTokenDb for ::salsa::Db {
    fn semantic_tokens_ext(
        &self,
        module_path: ModulePath,
        _range: Option<TextRange>,
    ) -> EntityTreeResult<&[ext::SemanticToken]> {
        Ok(semantic_tokens_ext_without_range(self, module_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }
}
