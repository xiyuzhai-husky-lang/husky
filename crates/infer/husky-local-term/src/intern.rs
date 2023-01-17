use std::marker::PhantomData;

use crate::*;

// abstraction

#[derive(Debug, PartialEq, Eq)]
pub struct LocalInterner<T: Eq>(Vec<T>);
pub struct LocalInternerIdx<T: Eq>(usize, PhantomData<T>);
impl<T: Eq> LocalInternerIdx<T> {
    fn new(raw: usize) -> Self {
        Self(raw, PhantomData)
    }
}

impl<T: Eq> LocalInterner<T> {
    fn intern(&mut self, t: T) -> LocalInternerIdx<T> {
        let raw = match self.0.iter().position(|s| s == &t) {
            Some(raw) => raw,
            None => {
                let raw = self.0.len();
                self.0.push(t);
                raw
            }
        };
        LocalInternerIdx::new(raw)
    }
}

impl<T: Eq> std::ops::Index<LocalInternerIdx<T>> for LocalInterner<T> {
    type Output = T;

    fn index(&self, index: LocalInternerIdx<T>) -> &Self::Output {
        &self.0[index.0]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LocalTermInterner {
    curries: LocalInterner<LocalTermCurry>,
    applications: LocalInterner<LocalTermApplication>,
    abstractions: LocalInterner<LocalTermAbstraction>,
    jordans: LocalInterner<LocalTermJordan>,
    subentities: LocalInterner<LocalTermSubentity>,
    as_trai_subentities: LocalInterner<LocalTermAsTraitSubentity>,
    trai_constraints: LocalInterner<LocalTermTraitConstraint>,
}

impl LocalTermInterner {
    pub(crate) fn intern_curry(&mut self, curry: LocalTermCurry) -> LocalTermCurryIdx {
        self.curries.intern(curry)
    }
    pub(crate) fn intern_application(
        &mut self,
        application: LocalTermApplication,
    ) -> LocalTermApplicationIdx {
        self.applications.intern(application)
    }
    pub(crate) fn intern_abstraction(
        &mut self,
        abstraction: LocalTermAbstraction,
    ) -> LocalTermAbstractionIdx {
        self.abstractions.intern(abstraction)
    }

    pub(crate) fn intern_jordan(&mut self, jordan: LocalTermJordan) -> LocalTermJordanIdx {
        self.jordans.intern(jordan)
    }

    pub(crate) fn intern_subentity(
        &mut self,
        subentity: LocalTermSubentity,
    ) -> LocalTermSubentityIdx {
        self.subentities.intern(subentity)
    }

    pub(crate) fn intern_as_trai_subentity(
        &mut self,
        as_trai_subentity: LocalTermAsTraitSubentity,
    ) -> LocalTermAsTraitSubentityIdx {
        self.as_trai_subentities.intern(as_trai_subentity)
    }

    pub(crate) fn intern_trai_constraint(
        &mut self,
        trai_constraint: LocalTermTraitConstraint,
    ) -> LocalTermTraitConstraintIdx {
        self.trai_constraints.intern(trai_constraint)
    }
}

pub type LocalTermCurryIdx = LocalInternerIdx<LocalTermCurry>;

impl std::ops::Index<LocalTermCurryIdx> for LocalTermInterner {
    type Output = LocalTermCurry;

    fn index(&self, index: LocalTermCurryIdx) -> &Self::Output {
        &self.curries[index]
    }
}

pub type LocalTermApplicationIdx = LocalInternerIdx<LocalTermApplication>;

impl std::ops::Index<LocalTermApplicationIdx> for LocalTermInterner {
    type Output = LocalTermApplication;

    fn index(&self, index: LocalTermApplicationIdx) -> &Self::Output {
        &self.applications[index]
    }
}

pub type LocalTermAbstractionIdx = LocalInternerIdx<LocalTermAbstraction>;

impl std::ops::Index<LocalTermAbstractionIdx> for LocalTermInterner {
    type Output = LocalTermAbstraction;

    fn index(&self, index: LocalTermAbstractionIdx) -> &Self::Output {
        &self.abstractions[index]
    }
}

pub type LocalTermJordanIdx = LocalInternerIdx<LocalTermJordan>;

impl std::ops::Index<LocalTermJordanIdx> for LocalTermInterner {
    type Output = LocalTermJordan;

    fn index(&self, index: LocalTermJordanIdx) -> &Self::Output {
        &self.jordans[index]
    }
}

pub type LocalTermSubentityIdx = LocalInternerIdx<LocalTermSubentity>;

impl std::ops::Index<LocalTermSubentityIdx> for LocalTermInterner {
    type Output = LocalTermSubentity;

    fn index(&self, index: LocalTermSubentityIdx) -> &Self::Output {
        &self.subentities[index]
    }
}

pub type LocalTermAsTraitSubentityIdx = LocalInternerIdx<LocalTermAsTraitSubentity>;

impl std::ops::Index<LocalTermAsTraitSubentityIdx> for LocalTermInterner {
    type Output = LocalTermAsTraitSubentity;

    fn index(&self, index: LocalTermAsTraitSubentityIdx) -> &Self::Output {
        &self.as_trai_subentities[index]
    }
}

pub type LocalTermTraitConstraintIdx = LocalInternerIdx<LocalTermTraitConstraint>;

impl std::ops::Index<LocalTermTraitConstraintIdx> for LocalTermInterner {
    type Output = LocalTermTraitConstraint;

    fn index(&self, index: LocalTermTraitConstraintIdx) -> &Self::Output {
        &self.trai_constraints[index]
    }
}
