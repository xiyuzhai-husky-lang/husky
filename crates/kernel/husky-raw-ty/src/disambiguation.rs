/// a raw_type path can be interpreted in two different ways:
///
/// - \[explicit curries to\] a raw_type
/// - \[explicit curries to\] a raw_type constructor
///
/// the final curry destination of the two different interpretation are different
///
/// for example, the raw_type of raw_type path `List` can either be
///
/// - `∀ universe u, explicit covariant Sort u -> Sort u`,
///
///     the final curry destination is in universe `Sort u`
/// - `∀ universe u, explicit covariant (E: Sort u) -> () -> List E`,
///
///     the final curry destination is in universe `List E`
/// disambiguate between raw_type itself (or template) and its instance or constructor
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypePathDisambiguation {
    RawTypeItselfOrTemplate,
    /// if raw_type is a unit struct, this will become an instance,
    ///
    /// otherwise constructor
    InstanceOrConstructor,
}
