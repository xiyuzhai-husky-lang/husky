use word::CustomIdentifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VMLoopKind {
    For {
        frame_var: CustomIdentifier,
        initial_boundary_kind: BoundaryKind,
        final_boundary_kind: BoundaryKind,
        step: LoopStep,
    },
    ForExt,
    While,
    DoWhile,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BoundaryKind {
    UpperOpen,
    UpperClosed,
    LowerOpen,
    LowerClosed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LoopStep(pub i32);

impl LoopStep {
    pub fn n(&self, a: i32, b: i32) -> i32 {
        if (b - a) * self.0 >= 0 {
            (b - a) / self.0 + 1
        } else {
            0
        }
    }

    pub fn frame_var(&self, a: i32, i: i32) -> i32 {
        a + self.0 * i
    }
}

#[test]
fn test_step_n_for_pos_step() {
    let step = LoopStep(1);
    assert_eq!(step.n(0, 0), 1);
    assert_eq!(step.n(0, 1), 2);
    assert_eq!(step.n(0, 2), 3);
    assert_eq!(step.n(0, -1), 0);
}

#[test]
fn test_step_n_for_neg_step() {
    let step = LoopStep(-1);
    assert_eq!(step.n(0, 0), 1);
    assert_eq!(step.n(0, -1), 2);
    assert_eq!(step.n(0, -2), 3);
    assert_eq!(step.n(0, -3), 4);
    assert_eq!(step.n(0, 1), 0);
}
