use husky_fluffy_term::FlyLifetime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirLifetime {}

impl HirLifetime {
    pub fn from_fluffy(_lifetime: FlyLifetime) -> Self {
        todo!()
    }
}
