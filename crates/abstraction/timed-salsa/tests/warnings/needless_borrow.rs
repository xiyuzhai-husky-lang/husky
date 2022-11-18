trait Db: timed_salsa::DbWithJar<Jar> {}

#[timed_salsa::jar(db = Db)]
struct Jar(TokenTree);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Token {}

impl timed_salsa::DebugWithDb<dyn Db + '_> for Token {
    fn fmt(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn Db,
        _include_all_fields: bool,
    ) -> std::fmt::Result {
        unreachable!()
    }
}

#[timed_salsa::tracked(jar = Jar)]
struct TokenTree {
    #[return_ref]
    tokens: Vec<Token>,
}
