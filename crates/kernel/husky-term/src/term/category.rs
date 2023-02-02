use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TermCategory {
    universe: TermUniverse,
}

impl TermCategory {
    pub fn new(universe: TermUniverse) -> Self {
        Self { universe }
    }
}

impl std::fmt::Display for TermCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.universe.raw() {
            0 => f.write_str("Prop"),
            1 => f.write_str("Type"),
            _ => todo!(),
        }
    }
}
// pub fn as_str(self) -> &'static str {
//     match self {
//         TermCategory::Type => "Type",
//         TermCategory::Sort => "Sort",
//         TermCategory::Prop => "Prop",
//     }
// }
