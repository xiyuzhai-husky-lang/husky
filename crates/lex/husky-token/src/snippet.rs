use crate::*;
use husky_vfs::script::Script;

#[salsa::tracked(jar = TokenJar, return_ref)]
pub(crate) fn tokenize_snippet(db: &::salsa::Db, snippet: Script) -> RangedTokenSheet {
    let input = snippet.data(db);
    lex_tracked(db, input)
}
