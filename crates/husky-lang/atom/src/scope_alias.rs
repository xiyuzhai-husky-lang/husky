pub type ScopeAliasStack = folded::FoldedStack<ScopeAlias>;

pub struct ScopeAlias {
    ident: word::Identifier,
    scope_id: scope::ScopeId,
}
