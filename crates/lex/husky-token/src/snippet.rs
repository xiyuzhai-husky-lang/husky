use crate::*;
use husky_vfs::chunk::Chunk;

#[salsa::tracked(jar = TokenJar, return_ref)]
pub(crate) fn tokenize_snippet(db: &::salsa::Db, snippet: Chunk) -> RangedTokenSheet {
    let input = snippet.data(db);
    lex_tracked(db, input)
}
