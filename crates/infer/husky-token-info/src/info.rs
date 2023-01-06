use husky_entity_path::EntityPath;
use husky_entity_taxonomy::EntityKind;
use husky_expr::{ExprSheet, VariableIdx, VariableSheet};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenInfo {
    None,
    Entity(EntityKind),
    ImplicitParameter,
    Parameter,
    Variable {
        variable_sheet: VariableSheet,
        variable_idx: VariableIdx,
    },
    Field,
    Method,
}

impl Default for TokenInfo {
    fn default() -> Self {
        TokenInfo::None
    }
}
