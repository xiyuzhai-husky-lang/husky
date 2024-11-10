use super::*;

/// only those visually different from the latin letters are listed here
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LxMathUpperGreekLetter {
    Gamma,
    Delta,
    Theta,
    Lambda,
    Xi,
    Pi,
    Sigma,
    Phi,
    Psi,
    Omega,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LxMathLowerGreekLetter {
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
    Zeta,
    Eta,
    Theta,
    Iota,
    Kappa,
    Lambda,
    Mu,
    Nu,
    Xi,
    Omicron,
    Pi,
    Rho,
    Sigma,
    Tau,
    Upsilon,
    Phi,
    Chi,
    Psi,
    Omega,
}

impl LxMathLetter {
    pub const UPPER_GAMMA: Self = LxMathLetter::UpperGreek(LxMathUpperGreekLetter::Gamma);
    pub const UPPER_DELTA: Self = LxMathLetter::UpperGreek(LxMathUpperGreekLetter::Delta);
    pub const UPPER_THETA: Self = LxMathLetter::UpperGreek(LxMathUpperGreekLetter::Theta);
    pub const UPPER_LAMBDA: Self = LxMathLetter::UpperGreek(LxMathUpperGreekLetter::Lambda);
    pub const UPPER_XI: Self = LxMathLetter::UpperGreek(LxMathUpperGreekLetter::Xi);
    pub const UPPER_PI: Self = LxMathLetter::UpperGreek(LxMathUpperGreekLetter::Pi);
    pub const UPPER_SIGMA: Self = LxMathLetter::UpperGreek(LxMathUpperGreekLetter::Sigma);
    pub const UPPER_PHI: Self = LxMathLetter::UpperGreek(LxMathUpperGreekLetter::Phi);
    pub const UPPER_PSI: Self = LxMathLetter::UpperGreek(LxMathUpperGreekLetter::Psi);
    pub const UPPER_OMEGA: Self = LxMathLetter::UpperGreek(LxMathUpperGreekLetter::Omega);

    pub const LOWER_ALPHA: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Alpha);
    pub const LOWER_BETA: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Beta);
    pub const LOWER_GAMMA: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Gamma);
    pub const LOWER_DELTA: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Delta);
    pub const LOWER_EPSILON: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Epsilon);
    pub const LOWER_ZETA: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Zeta);
    pub const LOWER_ETA: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Eta);
    pub const LOWER_THETA: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Theta);
    pub const LOWER_IOTA: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Iota);
    pub const LOWER_KAPPA: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Kappa);
    pub const LOWER_LAMBDA: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Lambda);
    pub const LOWER_MU: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Mu);
    pub const LOWER_NU: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Nu);
    pub const LOWER_XI: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Xi);
    pub const LOWER_OMICRON: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Omicron);
    pub const LOWER_PI: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Pi);
    pub const LOWER_RHO: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Rho);
    pub const LOWER_SIGMA: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Sigma);
    pub const LOWER_TAU: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Tau);
    pub const LOWER_UPSILON: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Upsilon);
    pub const LOWER_PHI: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Phi);
    pub const LOWER_CHI: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Chi);
    pub const LOWER_PSI: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Psi);
    pub const LOWER_OMEGA: Self = LxMathLetter::LowerGreek(LxMathLowerGreekLetter::Omega);
}
