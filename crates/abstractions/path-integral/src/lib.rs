pub mod state;

use self::state::*;

pub trait IsPathIntegralContext: Sized {
    type Node: Eq + Copy;
    type Weight: Eq;
    type Value: Clone;
    fn identity_weight(&self) -> Self::Weight;
    fn compose_weights(&self, a: &Self::Weight, b: &Self::Weight) -> Self::Weight;
    fn merge_weights(&self, a: &Self::Weight, b: Self::Weight) -> Self::Weight;
    /// cached.
    fn value(&self, node: Self::Node) -> &Self::Value;
    /// expected to be cached
    fn weighted_reaches(&self, node: Self::Node) -> &[(Self::Node, Self::Weight)];
    fn calc_full_reaches(&self) -> Vec<Self::Node> {
        let mut cache = FullReachCache::new(self);
        cache.populate_util_stable();
        cache.finish()
    }
    /// cached.
    ///
    /// equal to the result of `calc_full_reaches`
    ///
    /// the convention is that the first element is equal to node itself
    fn full_reaches(&self, node: Self::Node) -> &[Self::Node];
    fn integrate<'a>(
        &self,
        weighted_values: impl Iterator<Item = (&'a Self::Value, Self::Weight)>,
    ) -> Self::Value
    where
        Self::Value: 'a;
    /// expected to be a field access
    fn center(&self) -> Self::Node;
    /// final
    fn calc_integrated_value(&self) -> Self::Value {
        let mut cache = PathIntegralCache::new(self);
        cache.propagate_util_stable();
        cache.integrate()
    }
    /// cached
    fn integrated_value(&self, node: Self::Node) -> &Self::Value;

    /// equal to the result of `calc_integrated_value` if there isn't a path reaching from node to center;
    /// otherwise, equal to `value`
    fn smart_value(&self, node: Self::Node) -> (&Self::Value, bool) {
        if self.full_reaches(node).contains(&self.center()) {
            (self.value(node), true)
        } else {
            (self.integrated_value(node), false)
        }
    }
}

struct FullReachCache<'a, C: IsPathIntegralContext> {
    ctx: &'a C,
    full_reaches: Vec<C::Node>,
}

/// # constructor
impl<'a, C: IsPathIntegralContext> FullReachCache<'a, C> {
    fn new(ctx: &'a C) -> Self {
        let center = ctx.center();
        Self {
            ctx,
            full_reaches: vec![center],
        }
    }
}
/// actions

impl<'a, C: IsPathIntegralContext> FullReachCache<'a, C> {
    fn populate_util_stable(&mut self) {
        loop {
            let len = self.full_reaches.len();
            self.populate_from_new_nodes(len);
            if self.full_reaches.len() == len {
                break;
            }
        }
    }

    fn populate_from_new_nodes(&mut self, prev_len: usize) {
        let len = self.full_reaches.len();
        for source_index in 0..len {
            self.populate(source_index);
        }
    }

    fn populate(&mut self, source_index: usize) {
        let node = self.full_reaches[source_index];
        for &(reach, _) in self.ctx.weighted_reaches(node) {
            if !self.full_reaches.contains(&reach) {
                self.full_reaches.push(reach)
            }
        }
        todo!()
    }

    fn finish(self) -> Vec<C::Node> {
        self.full_reaches
    }
}

struct PathIntegralCache<'a, C: IsPathIntegralContext> {
    ctx: &'a C,
    weighted_values: Vec<(C::Node, &'a C::Value, TrackedWeight<C>)>,
    global_state: GlobalState,
}

/// # constructor
impl<'a, C: IsPathIntegralContext> PathIntegralCache<'a, C> {
    fn new(ctx: &'a C) -> Self {
        let center = ctx.center();
        Self {
            ctx,
            weighted_values: vec![(
                center,
                ctx.value(center),
                TrackedWeight::new(ctx.identity_weight(), true),
            )],
            global_state: Default::default(),
        }
    }
}

/// actions

impl<'a, C: IsPathIntegralContext> PathIntegralCache<'a, C> {
    fn propagate_util_stable(&mut self) {
        loop {
            self.global_state = Default::default();
            self.propagate_from_all_nonterminal_nodes();
            if !self.global_state.change_flag() {
                break;
            }
        }
    }

    fn propagate_from_all_nonterminal_nodes(&mut self) {
        let len = self.weighted_values.len();
        for source_index in 0..len {
            if self.weighted_values[source_index].2.try_propagate() {
                self.propagate(source_index);
            }
        }
    }

    fn propagate(&mut self, source_index: usize) {
        let node = self.weighted_values[source_index].0;
        let reaches = self.ctx.weighted_reaches(node);
        for &(reach, ref reach_weight) in reaches {
            // avoid self referencing
            if reach != self.ctx.center() {
                self.propagate_reach(source_index, reach, reach_weight)
            }
        }
    }

    fn propagate_reach(&mut self, source_index: usize, reach: C::Node, reach_weight: &C::Weight) {
        let tracked_weight = &self.weighted_values[source_index].2;
        let new_weight = self
            .ctx
            .compose_weights(tracked_weight.weight(), reach_weight);
        match self
            .weighted_values
            .iter_mut()
            .find(|&&mut (node, ..)| node == reach)
        {
            Some((_, _, tracked_weight)) => {
                tracked_weight.merge(new_weight, self.ctx, &mut self.global_state)
            }
            None => {
                let (value, is_nonterminal) = self.ctx.smart_value(reach);
                self.weighted_values.push((
                    reach,
                    value,
                    TrackedWeight::new(new_weight, is_nonterminal),
                ));
            }
        }
    }

    fn integrate(self) -> C::Value {
        let weighted_values = self
            .weighted_values
            .into_iter()
            .map(|(_, value, tracked_weight)| (value, tracked_weight.finish()));
        let integrated_value = self.ctx.integrate(weighted_values);
        integrated_value
    }
}
