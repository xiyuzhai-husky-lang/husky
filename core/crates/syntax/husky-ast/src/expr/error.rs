// use std::sync::Arc;

//

// use husky_text::TextRange;

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct ExprError {
//     pub range: TextRange,
//     pub kind: ExprErrorKind,
//     pub src: DevSource,
// }

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub enum ExprErrorKind {
//     BreakRule(ExprRule),
//     General(String),
// }

// impl From<String> for ExprErrorKind {
//     fn from(msg: String) -> Self {
//         Self::General(msg)
//     }
// }

// impl From<ExprRule> for ExprErrorKind {
//     fn from(rule: ExprRule) -> Self {
//         Self::BreakRule(rule)
//     }
// }

// impl ExprError {
//     pub fn new(range: TextRange, kind: ExprErrorKind, src: DevSource) -> ExprError {
//         Self { range, kind, src }
//     }
// }

// pub type ExprResult<T> = Result<T, ExprError>;
// pub type ExprResultArc<T> = Result<Arc<T>, ExprError>;

// impl From<&husky_atom::AtomError> for ExprError {
//     fn from(error: &husky_atom::AtomError) -> Self {
//         Self {
//             range: error.range.clone(),
//             kind: ExprErrorKind::AtomError(error.kind.clone()),
//             src: error.src,
//         }
//     }
// }

// impl From<husky_atom::AtomError> for ExprError {
//     fn from(error: husky_atom::AtomError) -> Self {
//         Self {
//             range: error.range,
//             kind: ExprErrorKind::AtomError(error.kind),
//             src: error.src,
//         }
//     }
// }

// impl From<husky_entity_route_syntax::ModuleFromFileError> for ExprError {
//     fn from(_: husky_entity_route_syntax::ModuleFromFileError) -> Self {
//         todo!()
//     }
// }

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum ExprRule {
//     ScopeShouldBeCalled,
//     BracketsShouldMatch,
//     BracketedExprCommaListShouldBeEitherCalledOrIndexed,
// }
