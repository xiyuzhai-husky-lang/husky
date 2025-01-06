use crate::*;
use ordered_float::NotNan;

pub struct MiracleState {
    vector: Vec<u64>,
    heartbeats: u64,
}

impl MiracleState {
    pub fn new() -> Self {
        Self {
            vector: vec![],
            heartbeats: 0,
        }
    }
}

impl MiracleState {
    pub fn norm(&self, metrics: &[MiracleMetric]) -> NotNan<f64> {
        metrics
            .iter()
            .map(|metric| metric.norm(&self.vector))
            .min()
            .expect("metrics should not be empty")
    }
}

pub(crate) fn calc_alt_option_with_new_value_appended<G, R>(
    g: &mut G,
    value: u64,
    mut f: impl FnMut(&mut G) -> AltOption<MiracleResult<R>>,
) -> AltOption<MiracleResult<R>>
where
    G: HasMiracle,
{
    g.miracle_mut().state_mut().vector.push(value);
    update_heartbeats(g, value);
    if g.miracle().state().heartbeats >= g.miracle().config().max_heartbeats {
        return AltSome(Err(MiracleError::HeartbeatsExceeded));
    }
    let result = f(g);
    g.miracle_mut().state_mut().vector.pop();
    result
}

/// We only increment the heartbeats if the value is positive.
///
/// There are two equivalent interpretations:
/// - the vector actually represents an infinite dimensional vector by zero paddings
/// - the number of leafs in a tree is equal to the number of nodes in the tree with its vector ending with 0,
///   where the vector representation of a node comes from the sibling indices of its ancestry.
fn update_heartbeats<G: HasMiracle>(g: &mut G, value: u64) {
    if value > 0 {
        g.miracle_mut().state_mut().heartbeats += 1;
    }
}
