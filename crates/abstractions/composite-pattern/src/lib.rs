pub trait AtomicPattern {
    type Input: ?Sized;
    fn contains(&self, t: &Self::Input) -> bool;
}

pub enum CompositePattern<P>
where
    P: AtomicPattern,
{
    Trivial { value: bool },
    Atom { atom: P },
    And { subpatterns: Vec<Self> },
    Or { subpatterns: Vec<Self> },
    NotAtom { atom: P },
    NotAnd { subpatterns: Vec<Self> },
    NotOr { subpatterns: Vec<Self> },
}

impl<P> From<P> for CompositePattern<P>
where
    P: AtomicPattern,
{
    fn from(atom: P) -> Self {
        CompositePattern::Atom { atom }
    }
}

impl<P> CompositePattern<P>
where
    P: AtomicPattern,
{
    pub fn contains(&self, input: &P::Input) -> bool {
        match self {
            CompositePattern::Trivial { value } => *value,
            CompositePattern::Atom { atom } => atom.contains(input),
            CompositePattern::And { subpatterns } => subpatterns
                .iter()
                .all(|subpattern| subpattern.contains(input)),
            CompositePattern::Or { subpatterns } => subpatterns
                .iter()
                .any(|subpattern| subpattern.contains(input)),
            CompositePattern::NotAtom { atom } => !atom.contains(input),
            CompositePattern::NotAnd { subpatterns } => !subpatterns
                .iter()
                .all(|subpattern| subpattern.contains(input)),
            CompositePattern::NotOr { subpatterns } => !subpatterns
                .iter()
                .any(|subpattern| subpattern.contains(input)),
        }
    }
}

impl<P> std::ops::BitOr for CompositePattern<P>
where
    P: AtomicPattern,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match self {
            CompositePattern::Trivial { value } => {
                if value {
                    // always true
                    self
                } else {
                    // always false
                    rhs
                }
            }
            CompositePattern::Or { mut subpatterns } => {
                subpatterns.push(rhs);
                CompositePattern::Or { subpatterns }
            }
            _ => match rhs {
                CompositePattern::Or { mut subpatterns } => {
                    subpatterns.push(self);
                    CompositePattern::Or { subpatterns }
                }
                _ => CompositePattern::Or {
                    subpatterns: vec![self, rhs],
                },
            },
        }
    }
}

impl<P> std::ops::BitAnd for CompositePattern<P>
where
    P: AtomicPattern,
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match self {
            CompositePattern::Trivial { value } => {
                if value {
                    // always true
                    rhs
                } else {
                    // always false
                    self
                }
            }
            CompositePattern::And { mut subpatterns } => {
                subpatterns.push(rhs);
                CompositePattern::And { subpatterns }
            }
            _ => match rhs {
                CompositePattern::Trivial { value } => {
                    if value {
                        // always true
                        self
                    } else {
                        // always false
                        rhs
                    }
                }
                CompositePattern::And { mut subpatterns } => {
                    subpatterns.push(self);
                    CompositePattern::And { subpatterns }
                }
                _ => CompositePattern::And {
                    subpatterns: vec![self, rhs],
                },
            },
        }
    }
}

impl<P> std::ops::Not for CompositePattern<P>
where
    P: AtomicPattern,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            CompositePattern::Trivial { value } => CompositePattern::Trivial { value: !value },
            CompositePattern::Atom { atom } => CompositePattern::NotAtom { atom },
            CompositePattern::Or { subpatterns } => CompositePattern::NotOr { subpatterns },
            CompositePattern::And { subpatterns } => CompositePattern::NotAnd { subpatterns },
            CompositePattern::NotAtom { atom } => CompositePattern::Atom { atom },
            CompositePattern::NotAnd { subpatterns } => CompositePattern::And { subpatterns },
            CompositePattern::NotOr { subpatterns } => CompositePattern::Or { subpatterns },
        }
    }
}
