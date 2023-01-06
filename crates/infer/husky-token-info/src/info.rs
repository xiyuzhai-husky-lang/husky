use husky_entity_path::EntityPath;
use husky_entity_taxonomy::EntityKind;
use husky_expr::{ExprSheet, VariableIdx};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenInfo {
    None,
    Entity(EntityKind),
    ImplicitParameter,
    Parameter,
    Variable {
        expr_sheet: ExprSheet,
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
