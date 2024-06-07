mod linear_graphs;

use crate::*;

#[salsa::jar]
pub struct Jar(
    self::linear_graphs::LinearGraphNode,
    self::linear_graphs::linear_graph_full_reaches,
);

#[salsa::db(Jar)]
pub struct DB();
