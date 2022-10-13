#[salsa::query_group(SymbolDbStorage)]
pub trait SymbolDb: SymbolQueries {}

pub trait SymbolQueries {}
