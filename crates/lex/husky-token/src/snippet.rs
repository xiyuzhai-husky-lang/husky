use crate::*;

#[salsa::input(db = TokenDb, jar = TokenJar)]
pub struct Snippet {
    #[return_ref]
    pub data: String,
}

#[salsa::tracked(jar = TokenJar, return_ref)]
pub(crate) fn tokenize_snippet(db: &dyn TokenDb, snippet: Snippet) -> RangedTokenSheet {
    let input = snippet.data(db);
    tokenize(db, input)
}
