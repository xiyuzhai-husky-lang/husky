use husky_entity_path::TypePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawTypeExpectation {
    FinalDestinationEqsSort,
    FinalDestinationEqsNonSortTypePath(TypePath),
}
