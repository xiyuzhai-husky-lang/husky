#[salsa::derive_debug_with_db(db = Db)]
struct A(usize, usize);

trait Db {}
