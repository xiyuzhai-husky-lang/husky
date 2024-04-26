use husky_lifetime_utils::capture::Captures;
use rustc_pattern_analysis::{
    constructor::{
        Constructor, ConstructorSet, IntRange, MaybeInfiniteInt, RangeEnd, VariantVisibility,
    },
    context::IsPatternAnalyisContext,
    usefulness::{PlaceValidity, UsefulnessReport},
    MatchArm, PrivateUninhabitedField,
};

/// Sets up `tracing` for easier debugging. Tries to look like the `rustc` setup.
pub fn init_tracing() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::Layer;
    let _ = tracing_tree::HierarchicalLayer::default()
        .with_writer(std::io::stderr)
        .with_indent_lines(true)
        .with_ansi(true)
        .with_targets(true)
        .with_indent_amount(2)
        .with_subscriber(
            tracing_subscriber::Registry::default()
                .with(tracing_subscriber::EnvFilter::from_default_env()),
        )
        .try_init();
}

/// A simple set of types.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Ty {
    /// Booleans
    Bool,
    /// 8-bit unsigned integers
    U8,
    /// Tuples.
    Tuple(&'static [Ty]),
    /// A struct with `arity` fields of type `ty`.
    BigStruct { arity: usize, ty: &'static Ty },
    /// A enum with `arity` variants of type `ty`.
    BigEnum { arity: usize, ty: &'static Ty },
}

/// The important logic.
impl Ty {
    pub fn sub_tys(&self, constructor: &Constructor<Ctx>) -> Vec<Self> {
        use Constructor::*;
        match (constructor, *self) {
            (Struct, Ty::Tuple(tys)) => tys.iter().copied().collect(),
            (Struct, Ty::BigStruct { arity, ty }) => (0..arity).map(|_| *ty).collect(),
            (Variant(_), Ty::BigEnum { ty, .. }) => vec![*ty],
            (Bool(..) | IntRange(..) | NonExhaustive | Missing | Wildcard, _) => vec![],
            _ => panic!("Unexpected constructor {constructor:?} for type {self:?}"),
        }
    }

    pub fn constructor_set(&self) -> ConstructorSet<Ctx> {
        match *self {
            Ty::Bool => ConstructorSet::Bool,
            Ty::U8 => ConstructorSet::Integers {
                range_1: IntRange::from_range(
                    MaybeInfiniteInt::new_finite_uint(0),
                    MaybeInfiniteInt::new_finite_uint(255),
                    RangeEnd::Included,
                ),
                range_2: None,
            },
            Ty::Tuple(..) | Ty::BigStruct { .. } => ConstructorSet::Struct { empty: false },
            Ty::BigEnum { arity, .. } => ConstructorSet::Variants {
                variants: (0..arity).map(|_| VariantVisibility::Visible).collect(),
                non_exhaustive: false,
            },
        }
    }

    pub fn write_variant_name(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        constructor: &Constructor<Ctx>,
    ) -> std::fmt::Result {
        match (*self, constructor) {
            (Ty::Tuple(..), _) => Ok(()),
            (Ty::BigStruct { .. }, _) => write!(f, "BigStruct"),
            (Ty::BigEnum { .. }, Constructor::Variant(i)) => write!(f, "BigEnum::Variant{i}"),
            _ => write!(f, "{:?}::{:?}", self, constructor),
        }
    }
}

/// Compute usefulness in our simple context (and set up tracing for easier debugging).
pub fn compute_match_usefulness<'p>(
    arms: &[MatchArm<'p, Ctx>],
    ty: Ty,
    scrut_validity: PlaceValidity,
    complexity_limit: Option<usize>,
) -> Result<UsefulnessReport<'p, Ctx>, ()> {
    init_tracing();
    rustc_pattern_analysis::usefulness::compute_match_usefulness(
        &Ctx,
        arms,
        ty,
        scrut_validity,
        complexity_limit,
    )
}

#[derive(Debug)]
pub struct Ctx;

/// The context for pattern analysis. Forwards anything interesting to `Ty` methods.
impl IsPatternAnalyisContext for Ctx {
    type Type = Ty;
    type Error = ();
    type VariantIdx = usize;
    type StringLiteral = ();
    type MatchArmData = ();
    type PatternDataExtra = ();

    fn is_exhaustive_patterns_feature_on(&self) -> bool {
        false
    }

    fn is_min_exhaustive_patterns_feature_on(&self) -> bool {
        false
    }

    fn constructor_arity(&self, constructor: &Constructor<Self>, ty: &Self::Type) -> usize {
        ty.sub_tys(constructor).len()
    }

    fn constructor_field_tys<'a>(
        &'a self,
        constructor: &'a Constructor<Self>,
        ty: &'a Self::Type,
    ) -> impl Iterator<Item = (Self::Type, PrivateUninhabitedField)> + ExactSizeIterator + Captures<'a>
    {
        ty.sub_tys(constructor)
            .into_iter()
            .map(|ty| (ty, PrivateUninhabitedField(false)))
    }

    fn constructors_for_ty(&self, ty: &Self::Type) -> Result<ConstructorSet<Self>, Self::Error> {
        Ok(ty.constructor_set())
    }

    fn write_variant_name(
        f: &mut std::fmt::Formatter<'_>,
        constructor: &Constructor<Self>,
        ty: &Self::Type,
    ) -> std::fmt::Result {
        ty.write_variant_name(f, constructor)
    }

    fn bug(&self, fmt: std::fmt::Arguments<'_>) -> Self::Error {
        panic!("{}", fmt)
    }

    /// Abort when reaching the complexity limit. This is what we'll check in tests.
    fn complexity_exceeded(&self) -> Result<(), Self::Error> {
        Err(())
    }
}

/// Construct a single pattern; see `pats!()`.
#[allow(unused_macros)]
macro_rules! pat {
    ($($rest:tt)*) => {{
        let mut vec = pats!($($rest)*);
        vec.pop().unwrap()
    }};
}

/// A macro to construct patterns. Called like `pats!(type_expr; pattern, pattern, ..)` and returns
/// a `Vec<DeconstructedPattern>`. A pattern can be nested and looks like `Constructor(pat, pat)` or
/// `Constructor { .i: pat, .j: pat }`, where `Constructor` is `Struct`, `Variant.i` (with index
/// `i`), as well as booleans and integer ranges.
///
/// The general structure of the macro is a tt-muncher with several stages identified with
/// `@something(args)`. The args are a key-value list (the keys ensure we don't mix the arguments
/// around) which is passed down and modified as needed. We then parse token-trees from
/// left-to-right. Non-trivial recursion happens when we parse the arguments to a pattern: we
/// recurse to parse the tokens inside `{..}`/`(..)`, and then we continue parsing anything that
/// follows.
macro_rules! pats {
    // Entrypoint
    // Parse `type; ..`
    ($ty:expr; $($rest:tt)*) => {{
        #[allow(unused_imports)]
        use rustc_pattern_analysis::{
            constructor::{Constructor, IntRange, MaybeInfiniteInt, RangeEnd},
            pattern::DeconstructedPattern,
        };
        let ty = $ty;
        // The heart of the macro is designed to push `IndexedPat`s into a `Vec`, so we work around
        // that.
        let sub_tys = ::std::iter::repeat(&ty);
        let mut vec = Vec::new();
        pats!(@constructor(vec:vec, sub_tys:sub_tys, idx:0) $($rest)*);
        vec.into_iter().map(|ipat| ipat.pat).collect::<Vec<_>>()
    }};

    // Parse `constructor ..`

    (@constructor($($args:tt)*) true $($rest:tt)*) => {{
        let constructor = Constructor::Bool(true);
        pats!(@pat($($args)*, constructor:constructor) $($rest)*)
    }};
    (@constructor($($args:tt)*) false $($rest:tt)*) => {{
        let constructor = Constructor::Bool(false);
        pats!(@pat($($args)*, constructor:constructor) $($rest)*)
    }};
    (@constructor($($args:tt)*) Struct $($rest:tt)*) => {{
        let constructor = Constructor::Struct;
        pats!(@pat($($args)*, constructor:constructor) $($rest)*)
    }};
    (@constructor($($args:tt)*) ( $($fields:tt)* ) $($rest:tt)*) => {{
        let constructor = Constructor::Struct; // tuples
        pats!(@pat($($args)*, constructor:constructor) ( $($fields)* ) $($rest)*)
    }};
    (@constructor($($args:tt)*) Variant.$variant:ident $($rest:tt)*) => {{
        let constructor = Constructor::Variant($variant);
        pats!(@pat($($args)*, constructor:constructor) $($rest)*)
    }};
    (@constructor($($args:tt)*) Variant.$variant:literal $($rest:tt)*) => {{
        let constructor = Constructor::Variant($variant);
        pats!(@pat($($args)*, constructor:constructor) $($rest)*)
    }};
    (@constructor($($args:tt)*) _ $($rest:tt)*) => {{
        let constructor = Constructor::Wildcard;
        pats!(@pat($($args)*, constructor:constructor) $($rest)*)
    }};

    // Integers and int ranges
    (@constructor($($args:tt)*) $($start:literal)?..$end:literal $($rest:tt)*) => {{
        let constructor = Constructor::IntRange(IntRange::from_range(
            pats!(@rangeboundary- $($start)?),
            pats!(@rangeboundary+ $end),
            RangeEnd::Excluded,
        ));
        pats!(@pat($($args)*, constructor:constructor) $($rest)*)
    }};
    (@constructor($($args:tt)*) $($start:literal)?.. $($rest:tt)*) => {{
        let constructor = Constructor::IntRange(IntRange::from_range(
            pats!(@rangeboundary- $($start)?),
            pats!(@rangeboundary+),
            RangeEnd::Excluded,
        ));
        pats!(@pat($($args)*, constructor:constructor) $($rest)*)
    }};
    (@constructor($($args:tt)*) $($start:literal)?..=$end:literal $($rest:tt)*) => {{
        let constructor = Constructor::IntRange(IntRange::from_range(
            pats!(@rangeboundary- $($start)?),
            pats!(@rangeboundary+ $end),
            RangeEnd::Included,
        ));
        pats!(@pat($($args)*, constructor:constructor) $($rest)*)
    }};
    (@constructor($($args:tt)*) $int:literal $($rest:tt)*) => {{
        let constructor = Constructor::IntRange(IntRange::from_range(
            pats!(@rangeboundary- $int),
            pats!(@rangeboundary+ $int),
            RangeEnd::Included,
        ));
        pats!(@pat($($args)*, constructor:constructor) $($rest)*)
    }};
    // Utility to manage range boundaries.
    (@rangeboundary $sign:tt $int:literal) => { MaybeInfiniteInt::new_finite_uint($int) };
    (@rangeboundary -) => { MaybeInfiniteInt::NegInfinity };
    (@rangeboundary +) => { MaybeInfiniteInt::PosInfinity };

    // Parse subfields: `(..)` or `{..}`

    // Constructor with no fields, e.g. `bool` or `Variant.1`.
    (@pat($($args:tt)*) $(,)?) => {
        pats!(@pat($($args)*) {})
    };
    (@pat($($args:tt)*) , $($rest:tt)*) => {
        pats!(@pat($($args)*) {}, $($rest)*)
    };
    // `(..)` and `{..}` are treated the same.
    (@pat($($args:tt)*) ( $($subpat:tt)* ) $($rest:tt)*) => {{
        pats!(@pat($($args)*) { $($subpat)* } $($rest)*)
    }};
    (@pat(vec:$vec:expr, sub_tys:$sub_tys:expr, idx:$idx:expr, constructor:$constructor:expr) { $($fields:tt)* } $($rest:tt)*) => {{
        let sub_tys = $sub_tys;
        let index = $idx;
        // Silly dance to work with both a vec and `iter::repeat()`.
        let ty = *(&sub_tys).clone().into_iter().nth(index).unwrap();
        let constructor = $constructor;
        let constructor_sub_tys = &ty.sub_tys(&constructor);
        #[allow(unused_mut)]
        let mut fields = Vec::new();
        // Parse subpatterns (note the leading comma).
        pats!(@fields(idx:0, vec:fields, sub_tys:constructor_sub_tys) ,$($fields)*);
        let arity = constructor_sub_tys.len();
        let pat = DeconstructedPattern::new(constructor, fields, arity, ty, ()).at_index(index);
        $vec.push(pat);

        // Continue parsing further patterns.
        pats!(@fields(idx:index+1, vec:$vec, sub_tys:sub_tys) $($rest)*);
    }};

    // Parse fields one by one.

    // No fields left.
    (@fields($($args:tt)*) $(,)?) => {};
    // `.i: pat` sets the current index to `i`.
    (@fields(idx:$_idx:expr, $($args:tt)*) , .$idx:literal : $($rest:tt)*) => {{
        pats!(@constructor($($args)*, idx:$idx) $($rest)*);
    }};
    (@fields(idx:$_idx:expr, $($args:tt)*) , .$idx:ident : $($rest:tt)*) => {{
        pats!(@constructor($($args)*, idx:$idx) $($rest)*);
    }};
    // Field without an explicit index; we use the current index which gets incremented above.
    (@fields(idx:$idx:expr, $($args:tt)*) , $($rest:tt)*) => {{
        pats!(@constructor($($args)*, idx:$idx) $($rest)*);
    }};
}
