use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomGroup {
    attr: GroupAttr,
    atoms: Vec<Atom>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct GroupAttr {
    keyword: Option<Keyword>,
    is_head: bool,
}

impl AtomGroup {
    pub(crate) fn new(keyword: Option<Keyword>, is_head: bool) -> Self {
        Self {
            attr: GroupAttr { keyword, is_head },
            atoms: Vec::new(),
        }
    }

    pub(crate) fn convexity(&self) -> Convexity {
        if let Some(atom) = self.atoms.last() {
            Convexity::right_side_convexity(&atom.kind)
        } else {
            Convexity::Concave
        }
    }

    pub(crate) fn is_convex(&self) -> bool {
        self.convexity() == Convexity::Convex
    }

    pub(crate) fn push(&mut self, atom: Atom) -> Result<(), AtomError> {
        if Convexity::compatible(self.convexity(), Convexity::left_side_convexity(&atom.kind)) {
            self.atoms.push(atom);
            Ok(())
        } else {
            Err(AtomError::new(atom.range, AtomRule::CompatibleConvexity))
        }
    }

    pub(crate) fn is_last(&self, kind: AtomKind) -> bool {
        if let Some(atom) = self.atoms.last() {
            atom.kind == kind
        } else {
            false
        }
    }

    pub fn attr(&self) -> GroupAttr {
        self.attr
    }
    pub fn atoms(&self) -> &[Atom] {
        &self.atoms
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Convexity {
    Convex,
    Concave,
}
impl Convexity {
    fn left_side_convexity(kind: &AtomKind) -> Convexity {
        match kind {
            AtomKind::Scope(_) | AtomKind::Variable(_) | AtomKind::Literal(_) => Convexity::Convex,
            AtomKind::Opr(opr) => match opr {
                Opr::Prefix(_) | Opr::Bra(_) => Convexity::Convex,
                Opr::Suffix(_) | Opr::Ket(_) => Convexity::Concave,
                Opr::Binary(_) | Opr::Join => Convexity::Concave,
            },
        }
    }

    fn right_side_convexity(kind: &AtomKind) -> Convexity {
        match kind {
            AtomKind::Scope(_) | AtomKind::Variable(_) | AtomKind::Literal(_) => Convexity::Convex,
            AtomKind::Opr(opr) => match opr {
                Opr::Suffix(_) | Opr::Ket(_) => Convexity::Convex,
                Opr::Prefix(_) | Opr::Bra(_) => Convexity::Concave,
                Opr::Binary(_) | Opr::Join => Convexity::Concave,
            },
        }
    }

    fn compatible(left: Convexity, right: Convexity) -> bool {
        match left {
            Convexity::Convex => match right {
                Convexity::Convex => false,
                Convexity::Concave => true,
            },
            Convexity::Concave => match right {
                Convexity::Convex => true,
                Convexity::Concave => false,
            },
        }
    }
}
