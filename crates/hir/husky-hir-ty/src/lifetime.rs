use husky_fluffy_term::FluffyLifetime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirLifetime {}

impl HirLifetime {
    pub fn from_fluffy(lifetime: FluffyLifetime) -> Self {
        todo!()
    }
}
