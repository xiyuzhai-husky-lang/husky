mod curry;

pub use self::curry::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermRitchieKind {
    Fp,
    Fn,
    FnMut,
}
