mod internal;
#[cfg(test)]
mod tests;

use std::{hash::Hash, sync::Arc};

use common::*;
use stdx::sync::ARwLock;

use internal::VersionControlInternal;

pub trait IssuerData: std::fmt::Debug + Clone + PartialEq + Eq {}

impl<T> IssuerData for T where T: std::fmt::Debug + Clone + PartialEq + Eq {}

#[derive(Debug, Clone)]
pub struct VersionControl<NameId, T>
where
    NameId: Hash + Copy + Eq,
    T: IssuerData,
{
    internal: ARwLock<VersionControlInternal<NameId, T>>,
}
impl<NameId, T> VersionControl<NameId, T>
where
    NameId: Hash + Copy + Eq,
    T: IssuerData,
{
    pub fn new() -> Self {
        Self {
            internal: ARwLock::new(VersionControlInternal::new()),
        }
    }

    pub fn update(&self, name: NameId, data: Arc<T>, dependees: &[NameId]) -> Uid {
        self.internal
            .write(|internal| internal.update(name, data, dependees))
    }

    pub fn uid(&self, name: NameId) -> Uid {
        self.internal.write(|internal| internal.uid(name))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uid {
    raw: usize,
}
