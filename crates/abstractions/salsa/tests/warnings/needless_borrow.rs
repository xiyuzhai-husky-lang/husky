#[salsa::jar(db = Db)]
struct Jar(TokenTree);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Token {}

impl salsa::DebugWithDb for Token {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>, _db: &::salsa::Db) -> std::fmt::Result {
        unreachable!()
    }
}

#[salsa::tracked(db = Db, jar = Jar)]
struct TokenTree {
    #[return_ref]
    tokens: Vec<Token>,
}
