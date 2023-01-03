use crate::*;

#[salsa::input(jar = TokenJar)]
pub struct Snippet {
    #[return_ref]
    pub data: String,
}

#[salsa::tracked(jar = TokenJar, return_ref)]
pub(crate) fn tokenize_snippet(db: &dyn TokenDb, snippet: Snippet) -> TokenSheet {
    let input = snippet.data(db);
    TokenSheet::new(tokenize(db, input))
}
