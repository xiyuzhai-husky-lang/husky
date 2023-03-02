use crate::*;

/// a type path can be interpreted in two different ways:
///
/// - \[explicit curries to\] a type
/// - \[explicit curries to\] a type constructor
///
/// the final curry destination of the two different interpretation are different
///
/// for example, the type of type path `List` can either be
///
/// - `∀ universe u, explicit covariant Sort u -> Sort u`,
///
///     the final curry destination is in universe `Sort u`
/// - `∀ universe u, explicit covariant (E: Sort u) -> () -> List E`,
///
///     the final curry destination is in universe `List E`
pub enum EntityPathTypeExpectation {
    Any,
    NoneDerived,
    NoneOriginal,
    FinalCurryDestinationEqsSort,
    FinalCurryDestinationEqsTypePathOrItsApplication { ty_path: TypePath },
}

impl EntityPathTypeExpectation {
    // #[inline(always)]
    // pub const fn new_expect_applicable_or_callable() -> Self {
    //     Self {
    //         expect_applicable_or_callable: true,
    //     }
    // }

    // pub(crate) fn expect_applicable_or_callable(&self) -> bool {
    //     self.expect_applicable_or_callable
    // }
}
