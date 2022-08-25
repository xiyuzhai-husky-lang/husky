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
    All { subpatterns: Vec<Self> },
    Any { subpatterns: Vec<Self> },
    NotAtom { atom: P },
    NotAll { subpatterns: Vec<Self> },
    NotAny { subpatterns: Vec<Self> },
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
            CompositePattern::All { subpatterns } => subpatterns
                .iter()
                .all(|subpattern| subpattern.contains(input)),
            CompositePattern::Any { subpatterns } => subpatterns
                .iter()
                .any(|subpattern| subpattern.contains(input)),
            CompositePattern::NotAtom { atom } => !atom.contains(input),
            CompositePattern::NotAll { subpatterns } => !subpatterns
                .iter()
                .all(|subpattern| subpattern.contains(input)),
            CompositePattern::NotAny { subpatterns } => !subpatterns
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
            CompositePattern::Atom { atom } => todo!(),
            CompositePattern::All { subpatterns } => todo!(),
            CompositePattern::Any { subpatterns } => todo!(),
            CompositePattern::NotAtom { atom } => todo!(),
            CompositePattern::NotAny { subpatterns } => todo!(),
            CompositePattern::NotAll { subpatterns } => todo!(),
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
            CompositePattern::Atom { atom } => todo!(),
            CompositePattern::All { subpatterns } => todo!(),
            CompositePattern::Any { subpatterns } => todo!(),
            CompositePattern::NotAtom { atom } => todo!(),
            CompositePattern::NotAny { subpatterns } => todo!(),
            CompositePattern::NotAll { subpatterns } => todo!(),
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
            CompositePattern::Trivial { value } => todo!(),
            CompositePattern::Atom { atom } => CompositePattern::NotAtom { atom },
            CompositePattern::Any { subpatterns } => CompositePattern::NotAny { subpatterns },
            CompositePattern::All { subpatterns } => CompositePattern::NotAll { subpatterns },
            CompositePattern::NotAtom { atom } => CompositePattern::Atom { atom },
            CompositePattern::NotAll { subpatterns } => CompositePattern::All { subpatterns },
            CompositePattern::NotAny { subpatterns } => CompositePattern::Any { subpatterns },
        }
    }
}
