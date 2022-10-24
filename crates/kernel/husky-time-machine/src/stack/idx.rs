use std::fmt::Pointer;

use super::*;

pub enum ResourceIdx {
    Variable(VariableIdx),
    Lifetime(LifetimeIdx),
}

pub struct VariableIdx(usize);
pub struct LifetimeIdx(usize);

impl ResourceStack {
    pub(crate) fn new_variable(&mut self, qual: VariableQualifier) -> VariableIdx {
        let idx = VariableIdx(self.0.len());
        self.0.push(Resource::Variable(VariableResource::new(qual)));
        idx
    }

    pub(crate) fn new_lifetime(&mut self) -> LifetimeIdx {
        let idx = LifetimeIdx(self.0.len());
        self.0.push(Resource::Lifetime(LifetimeResource::new()));
        idx
    }
}

// trivia

impl std::ops::Index<VariableIdx> for ResourceStack {
    type Output = VariableResource;

    fn index(&self, index: VariableIdx) -> &Self::Output {
        match self.0[index.0] {
            Resource::Variable(ref v) => v,
            Resource::Lifetime(_) => unreachable!(),
        }
    }
}

impl std::ops::Index<LifetimeIdx> for ResourceStack {
    type Output = LifetimeResource;

    fn index(&self, index: LifetimeIdx) -> &Self::Output {
        match self.0[index.0] {
            Resource::Variable(_) => unreachable!(),
            Resource::Lifetime(ref l) => l,
        }
    }
}

impl From<VariableIdx> for ResourceIdx {
    fn from(value: VariableIdx) -> Self {
        ResourceIdx::Variable(value)
    }
}

impl From<LifetimeIdx> for ResourceIdx {
    fn from(value: LifetimeIdx) -> Self {
        ResourceIdx::Lifetime(value)
    }
}

impl std::fmt::Display for ResourceIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceIdx::Variable(v) => v.fmt(f),
            ResourceIdx::Lifetime(l) => l.fmt(f),
        }
    }
}

impl std::fmt::Display for VariableIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "#".fmt(f)?;
        self.0.fmt(f)
    }
}
impl std::fmt::Display for LifetimeIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "'#".fmt(f)?;
        self.0.fmt(f)
    }
}
