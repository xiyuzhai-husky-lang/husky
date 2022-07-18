#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Convexity {
    Convex,
    Concave,
}
impl Convexity {
    pub fn compatible_with(self: Convexity, right: Convexity) -> bool {
        match self {
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
