use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpCsvExpr {
    String(String),
    Number(OrderedFloat<f64>),
    List(Vec<LpCsvExpr>),
}
