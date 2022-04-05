// use std::sync::Arc;

// 

// use file::FileError;
// use text::TextRange;

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct AtomError {
//     pub range: TextRange,
//     pub kind: AtomErrorKind,
//     pub src: DevSource,
// }

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub enum AtomErrorKind {
//     BreakRule(AtomRule),
//     FailExpectation(String),
// }

// impl From<AtomRule> for AtomErrorKind {
//     fn from(rule: AtomRule) -> Self {
//         Self::BreakRule(rule)
//     }
// }

// impl From<&'static str> for AtomErrorKind {
//     fn from(expectation: &'static str) -> Self {
//         Self::FailExpectation(expectation.into())
//     }
// }

// impl From<String> for AtomErrorKind {
//     fn from(expectation: String) -> Self {
//         Self::FailExpectation(expectation.into())
//     }
// }

// macro_rules! atom_error {
//     ($range: expr, $kind: expr) => {{
//         AtomError {
//             range: $range.into(),
//             kind: $kind.into(),
//             src: common::DevSource {
//                 file: file!(),
//                 line: line!(),
//             },
//         }
//     }};
//     ($range: expr, $kind: expr,) => {{
//         AtomError {
//             range: $range,
//             kind: $kind.into(),
//             src: common::DevSource {
//                 file: file!(),
//                 line: line!(),
//             },
//         }
//     }};
// }
// pub(crate) use atom_error;

// macro_rules! atom_err {
//     ($range: expr, $kind: expr) => {{
//         Err(AtomError {
//             range: $range.clone(),
//             kind: $kind.into(),
//             src: common::DevSource {
//                 file: file!(),
//                 line: line!(),
//             },
//         })
//     }};
//     ($range: expr, $kind: expr,) => {{
//         Err(AtomError {
//             range: $range,
//             kind: $kind.into(),
//             src: common::DevSource {
//                 file: file!(),
//                 line: line!(),
//             },
//         })
//     }};
// }
// pub(crate) use atom_err;

// impl From<FileError> for AtomError {
//     fn from(_: FileError) -> Self {
//         todo!()
//     }
// }

// impl From<entity_route::ModuleFromFileError> for AtomError {
//     fn from(_: entity_route::ModuleFromFileError) -> Self {
//         todo!()
//     }
// }

// pub type AstResult<T> = Result<T, AtomError>;
// pub type AstResultArc<T> = Result<Arc<T>, AtomError>;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum AtomRule {
//     BeforeColonShouldBeScope,
//     ListShouldBeWellFormed,
//     ScopeShouldExist,
//     AfterColonShouldBeCustomIdentifier,
//     NonEmptyAngles,
//     ExpectCommaInAngle,
//     AfterLAngleShouldBeCommaListOfScopes,
//     KeywordShouldBeAtStart,
//     CompatibleConvexity,
//     ExpectTypeAfterLightArrow,
//     BracketsShouldMatch,
//     OnlyTypesInTheParenthesisBeforeLightArrow,
// }
