#[salsa::query_group(TypeQueryStorage)]
pub trait ScopeSalsaQuery: scope::ScopeQuery + InternType {}
