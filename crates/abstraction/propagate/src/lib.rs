mod engine;
mod error;
#[cfg(test)]
mod tests;

pub use engine::*;
pub use error::*;

#[cfg(test)]
use tests::*;

pub trait Graph {
    type Value: Eq;
    fn len(&self) -> usize;
    fn dependencies(&self, idx: usize) -> &[usize];
    fn value_mut(&mut self, idx: usize) -> &mut Self::Value;
    fn eval(&self, idx: usize) -> Self::Value;
}

pub trait Propagate: Graph {
    fn update(&mut self, idx: usize) -> bool;
    fn propagate(&mut self, version_limit: usize) -> PropagationResult<()>;
}

impl<G> Propagate for G
where
    G: Graph,
{
    /// returns a flag indicating whether value has been changed
    fn update(&mut self, idx: usize) -> bool {
        let new_value = self.eval(idx);
        let old_value = self.value_mut(idx);
        let changed = &new_value != old_value;
        if changed {
            *old_value = new_value
        }
        changed
    }

    fn propagate(&mut self, version_limit: usize) -> PropagationResult<()> {
        let mut engine = PropagationEngine::new(self);
        let mut prev_version = 0;
        while prev_version < engine.max_version() {
            if prev_version > version_limit {
                return Err(PropagationError::InfiniteLoop);
            }
            prev_version = engine.max_version();
            engine.update_all()
        }
        Ok(())
    }
}
