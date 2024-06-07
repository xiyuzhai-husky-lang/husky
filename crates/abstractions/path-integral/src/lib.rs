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
    fn integrate(
        &self,
        weighted_values: &[(Self::Node, Self::Value, bool, Self::Weight)],
    ) -> Self::Value;
    /// expected to be a field access
    fn center(&self) -> Self::Node;
    /// final
    fn calc_integrated_value(
        &self,
    ) -> (
        Self::Value,
        Vec<(Self::Node, Self::Value, bool, Self::Weight)>,
    ) {
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
            (self.value(node), false)
        } else {
            (self.integrated_value(node), true)
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
    weighted_values: Vec<(C::Node, C::Value, bool, C::Weight)>,
    changed_flag: bool,
}

/// # constructor
impl<'a, C: IsPathIntegralContext> PathIntegralCache<'a, C> {
    fn new(ctx: &'a C) -> Self {
        let center = ctx.center();
        Self {
            ctx,
            weighted_values: vec![(
                center,
                ctx.value(center).clone(),
                false,
                ctx.identity_weight(),
            )],
            changed_flag: false,
        }
    }
}

/// actions

impl<'a, P: IsPathIntegralContext> PathIntegralCache<'a, P> {
    fn propagate_util_stable(&mut self) {
        loop {
            self.changed_flag = false;
            self.propagate_from_all_nonterminal_nodes();
            if !self.changed_flag {
                break;
            }
        }
    }

    fn propagate_from_all_nonterminal_nodes(&mut self) {
        let len = self.weighted_values.len();
        for source_index in 0..len {
            if !self.weighted_values[source_index].2 {
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

    fn propagate_reach(&mut self, source_index: usize, reach: P::Node, reach_weight: &P::Weight) {
        let weight = &self.weighted_values[source_index].3;
        let new_weight = self.ctx.compose_weights(weight, reach_weight);
        match self
            .weighted_values
            .iter_mut()
            .find(|&&mut (node, ..)| node == reach)
        {
            Some((_, _, _, old_weight)) => {
                let new_weight = self.ctx.merge_weights(old_weight, new_weight);
                if new_weight != *old_weight {
                    self.changed_flag = true;
                }
                *old_weight = new_weight
            }
            None => {
                let (value, terminal) = self.ctx.smart_value(reach);
                self.weighted_values
                    .push((reach, value.clone(), terminal, new_weight));
            }
        }
    }

    fn integrate(self) -> (P::Value, Vec<(P::Node, P::Value, bool, P::Weight)>) {
        let integrated_value = self.ctx.integrate(&self.weighted_values);
        (integrated_value, self.weighted_values)
    }
}
