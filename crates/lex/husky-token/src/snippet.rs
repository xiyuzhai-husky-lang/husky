use crate::*;
use husky_vfs::snippet::Snippet;

#[salsa::tracked(jar = TokenJar, return_ref)]
pub(crate) fn tokenize_snippet(db: &dyn TokenDb, snippet: Snippet) -> RangedTokenSheet {
    let input = snippet.data(db);
    tokenize(db, input)
}
