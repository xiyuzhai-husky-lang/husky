use husky_fly_term::FlyLifetime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirLifetime {}

impl HirLifetime {
    pub fn from_fly(_lifetime: FlyLifetime) -> Self {
        todo!()
    }
}
