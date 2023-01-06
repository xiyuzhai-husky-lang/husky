use husky_entity_taxonomy::EntityKind;
use husky_expr::VariableIdx;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenInfo {
    None,
    Entity(EntityKind),
    ImplicitParameter,
    Parameter,
    Variable { variable_idx: Option<VariableIdx> },
    Field,
    Method,
}

impl Default for TokenInfo {
    fn default() -> Self {
        TokenInfo::None
    }
}
