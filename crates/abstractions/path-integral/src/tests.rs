mod circular_graph;
mod linear_graph;

use crate::*;

#[salsa::jar]
pub struct Jar(
    self::linear_graph::LinearGraphNode,
    self::linear_graph::linear_graph_full_reaches,
    self::linear_graph::linear_graph_integrated_value,
    self::circular_graph::CircularGraphNode,
    self::circular_graph::circular_graph_full_reaches,
    self::circular_graph::circular_graph_integrated_value,
);

#[salsa::db(Jar)]
pub struct DB();
