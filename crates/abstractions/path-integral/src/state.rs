use crate::*;

pub(crate) struct GlobalState {
    change_flag: bool,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self { change_flag: false }
    }
}

pub struct TrackedWeight<C: IsPathIntegralContext> {
    weight: C::Weight,
    is_nonterminal: bool,
    can_propagate: bool,
}

/// # constructor
impl<C: IsPathIntegralContext> TrackedWeight<C> {
    pub(crate) fn new(weight: C::Weight, is_nonterminal: bool) -> Self {
        Self {
            weight,
            is_nonterminal,
            can_propagate: is_nonterminal,
        }
    }
}

/// # getters
impl GlobalState {
    pub(crate) fn change_flag(&self) -> bool {
        self.change_flag
    }
}

impl<C: IsPathIntegralContext> TrackedWeight<C> {
    pub fn weight(&self) -> &C::Weight {
        &self.weight
    }

    pub fn is_nonterminal(&self) -> bool {
        self.is_nonterminal
    }
}

/// # actions
impl<C: IsPathIntegralContext> TrackedWeight<C> {
    pub fn try_propagate(&mut self) -> bool {
        std::mem::replace(&mut self.can_propagate, false)
    }

    pub(crate) fn merge(&mut self, new_weight: C::Weight, ctx: &C, global_state: &mut GlobalState) {
        let new_weight = ctx.merge_weights(self.weight(), new_weight);
        if new_weight != self.weight {
            if self.is_nonterminal {
                self.can_propagate = true
            }
            global_state.change_flag = true;
            self.weight = new_weight;
        }
    }

    pub(crate) fn finish(self) -> C::Weight {
        self.weight
    }
}
