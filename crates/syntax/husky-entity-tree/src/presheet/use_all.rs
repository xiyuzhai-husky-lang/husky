use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UseAllTracker {
    parent: EntityPath,
    // how many symbols have been checked
    progress: usize,
}
