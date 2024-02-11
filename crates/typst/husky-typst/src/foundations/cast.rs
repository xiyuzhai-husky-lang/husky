use std::borrow::Cow;
use std::fmt::Write;
use std::hash::Hash;
use std::ops::Add;

use comemo::Prehashed;
use ecow::{eco_format, EcoString};
use smallvec::SmallVec;
use unicode_math_class::MathClass;

use crate::diag::{At, SourceResult, StrResult};
use crate::foundations::{array, repr, Packed, Repr, Str, Type, TypstElement, TypstValue};
use crate::syntax::{Span, Spanned};

#[rustfmt::skip]
#[doc(inline)]
pub use husky_typst_macros::{cast, Cast};

/// Determine details of a type.
///
/// Type casting works as follows:
/// - [`Reflect for T`](Reflect) describes the possible Typst values for `T`
///    (for documentation and autocomplete).
/// - [`IntoTypstValue for T`](IntoTypstValue) is for conversion from `T -> TypstValue`
///   (infallible)
/// - [`FromTypstValue for T`](FromTypstValue) is for conversion from `TypstValue -> T`
///   (fallible).
///
/// We can't use `TryFrom<TypstValue>` due to conflicting impls. We could use
/// `From<T> for TypstValue`, but that inverses the impl and leads to tons of
/// `.into()` all over the place that become hard to decipher.
pub trait Reflect {
    /// Describe what can be cast into this value.
    fn input() -> CastInfo;

    /// Describe what this value can be cast into.
    fn output() -> CastInfo;

    /// Whether the given value can be converted to `T`.
    ///
    /// This exists for performance. The check could also be done through the
    /// [`CastInfo`], but it would be much more expensive (heap allocation +
    /// dynamic checks instead of optimized machine code for each type).
    fn castable(value: &TypstValue) -> bool;

    /// Produce an error message for an inacceptable value type.
    ///
    /// ```
    /// assert_eq!(
    ///   <Int as Reflect>::error(&TypstValue::None),
    ///   "expected integer, found none",
    /// );
    /// ```
    fn error(found: &TypstValue) -> EcoString {
        Self::input().error(found)
    }
}

impl Reflect for TypstValue {
    fn input() -> CastInfo {
        CastInfo::Any
    }

    fn output() -> CastInfo {
        CastInfo::Any
    }

    fn castable(_: &TypstValue) -> bool {
        true
    }
}

impl<T: Reflect> Reflect for Spanned<T> {
    fn input() -> CastInfo {
        T::input()
    }

    fn output() -> CastInfo {
        T::output()
    }

    fn castable(value: &TypstValue) -> bool {
        T::castable(value)
    }
}

impl<T: TypstElement + Reflect> Reflect for Packed<T> {
    fn input() -> CastInfo {
        T::input()
    }

    fn output() -> CastInfo {
        T::output()
    }

    fn castable(value: &TypstValue) -> bool {
        T::castable(value)
    }
}

impl<T: Reflect> Reflect for Prehashed<T> {
    fn input() -> CastInfo {
        T::input()
    }

    fn output() -> CastInfo {
        T::output()
    }

    fn castable(value: &TypstValue) -> bool {
        T::castable(value)
    }
}

impl<T: Reflect> Reflect for StrResult<T> {
    fn input() -> CastInfo {
        T::input()
    }

    fn output() -> CastInfo {
        T::output()
    }

    fn castable(value: &TypstValue) -> bool {
        T::castable(value)
    }
}

impl<T: Reflect> Reflect for SourceResult<T> {
    fn input() -> CastInfo {
        T::input()
    }

    fn output() -> CastInfo {
        T::output()
    }

    fn castable(value: &TypstValue) -> bool {
        T::castable(value)
    }
}

impl<T: Reflect> Reflect for &T {
    fn input() -> CastInfo {
        T::input()
    }

    fn output() -> CastInfo {
        T::output()
    }

    fn castable(value: &TypstValue) -> bool {
        T::castable(value)
    }
}

impl<T: Reflect> Reflect for &mut T {
    fn input() -> CastInfo {
        T::input()
    }

    fn output() -> CastInfo {
        T::output()
    }

    fn castable(value: &TypstValue) -> bool {
        T::castable(value)
    }
}

/// Cast a Rust type into a Typst [`TypstValue`].
///
/// See also: [`Reflect`].
pub trait IntoTypstValue {
    /// Cast this type into a value.
    fn into_value(self) -> TypstValue;
}

impl IntoTypstValue for TypstValue {
    fn into_value(self) -> TypstValue {
        self
    }
}

impl IntoTypstValue for (&Str, &TypstValue) {
    fn into_value(self) -> TypstValue {
        TypstValue::Array(array![self.0.clone(), self.1.clone()])
    }
}

impl<T: IntoTypstValue + Clone> IntoTypstValue for Cow<'_, T> {
    fn into_value(self) -> TypstValue {
        self.into_owned().into_value()
    }
}

impl<T: TypstElement + IntoTypstValue> IntoTypstValue for Packed<T> {
    fn into_value(self) -> TypstValue {
        TypstValue::Content(self.pack())
    }
}

impl<T: IntoTypstValue> IntoTypstValue for Spanned<T> {
    fn into_value(self) -> TypstValue {
        self.v.into_value()
    }
}

impl<T: IntoTypstValue + Hash + 'static> IntoTypstValue for Prehashed<T> {
    fn into_value(self) -> TypstValue {
        self.into_inner().into_value()
    }
}

/// Cast a Rust type or result into a [`SourceResult<TypstValue>`].
///
/// Converts `T`, [`StrResult<T>`], or [`SourceResult<T>`] into
/// [`SourceResult<TypstValue>`] by `Ok`-wrapping or adding span information.
pub trait IntoResult {
    /// Cast this type into a value.
    fn into_result(self, span: Span) -> SourceResult<TypstValue>;
}

impl<T: IntoTypstValue> IntoResult for T {
    fn into_result(self, _: Span) -> SourceResult<TypstValue> {
        Ok(self.into_value())
    }
}

impl<T: IntoTypstValue> IntoResult for StrResult<T> {
    fn into_result(self, span: Span) -> SourceResult<TypstValue> {
        self.map(IntoTypstValue::into_value).at(span)
    }
}

impl<T: IntoTypstValue> IntoResult for SourceResult<T> {
    fn into_result(self, _: Span) -> SourceResult<TypstValue> {
        self.map(IntoTypstValue::into_value)
    }
}

impl<T: IntoTypstValue> IntoTypstValue for fn() -> T {
    fn into_value(self) -> TypstValue {
        self().into_value()
    }
}

/// Try to cast a Typst [`TypstValue`] into a Rust type.
///
/// See also: [`Reflect`].
pub trait FromTypstValue<V = TypstValue>: Sized + Reflect {
    /// Try to cast the value into an instance of `Self`.
    fn from_value(value: V) -> StrResult<Self>;
}

impl FromTypstValue for TypstValue {
    fn from_value(value: TypstValue) -> StrResult<Self> {
        Ok(value)
    }
}

impl<T: TypstElement + FromTypstValue> FromTypstValue for Packed<T> {
    fn from_value(mut value: TypstValue) -> StrResult<Self> {
        if let TypstValue::Content(content) = value {
            match content.into_packed::<T>() {
                Ok(packed) => return Ok(packed),
                Err(content) => value = TypstValue::Content(content),
            }
        }
        let val = T::from_value(value)?;
        Ok(Packed::new(val))
    }
}

impl<T: FromTypstValue + Hash + 'static> FromTypstValue for Prehashed<T> {
    fn from_value(value: TypstValue) -> StrResult<Self> {
        Ok(Self::new(T::from_value(value)?))
    }
}

impl<T: FromTypstValue> FromTypstValue<Spanned<TypstValue>> for T {
    fn from_value(value: Spanned<TypstValue>) -> StrResult<Self> {
        T::from_value(value.v)
    }
}

impl<T: FromTypstValue> FromTypstValue<Spanned<TypstValue>> for Spanned<T> {
    fn from_value(value: Spanned<TypstValue>) -> StrResult<Self> {
        let span = value.span;
        T::from_value(value.v).map(|t| Spanned::new(t, span))
    }
}

/// Describes a possible value for a cast.
#[derive(Debug, Clone, PartialEq, Hash, PartialOrd)]
pub enum CastInfo {
    /// Any value is okay.
    Any,
    /// A specific value, plus short documentation for that value.
    TypstValue(TypstValue, &'static str),
    /// Any value of a type.
    Type(Type),
    /// Multiple alternatives.
    Union(Vec<Self>),
}

impl CastInfo {
    /// Produce an error message describing what was expected and what was
    /// found.
    pub fn error(&self, found: &TypstValue) -> EcoString {
        let mut matching_type = false;
        let mut parts = vec![];

        self.walk(|info| match info {
            CastInfo::Any => parts.push("anything".into()),
            CastInfo::TypstValue(value, _) => {
                parts.push(value.repr());
                if value.ty() == found.ty() {
                    matching_type = true;
                }
            }
            CastInfo::Type(ty) => parts.push(eco_format!("{ty}")),
            CastInfo::Union(_) => {}
        });

        let mut msg = String::from("expected ");
        if parts.is_empty() {
            msg.push_str(" nothing");
        }

        msg.push_str(&repr::separated_list(&parts, "or"));

        if !matching_type {
            msg.push_str(", found ");
            write!(msg, "{}", found.ty()).unwrap();
        }

        if let TypstValue::Int(i) = found {
            if parts.iter().any(|p| p == "length") && !matching_type {
                write!(msg, ": a length needs a unit - did you mean {i}pt?").unwrap();
            }
        }

        msg.into()
    }

    /// Walk all contained non-union infos.
    pub fn walk<F>(&self, mut f: F)
    where
        F: FnMut(&Self),
    {
        fn inner<F>(info: &CastInfo, f: &mut F)
        where
            F: FnMut(&CastInfo),
        {
            if let CastInfo::Union(infos) = info {
                for child in infos {
                    inner(child, f);
                }
            } else {
                f(info);
            }
        }

        inner(self, &mut f)
    }
}

impl Add for CastInfo {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::Union(match (self, rhs) {
            (Self::Union(mut lhs), Self::Union(rhs)) => {
                for cast in rhs {
                    if !lhs.contains(&cast) {
                        lhs.push(cast);
                    }
                }
                lhs
            }
            (Self::Union(mut lhs), rhs) => {
                if !lhs.contains(&rhs) {
                    lhs.push(rhs);
                }
                lhs
            }
            (lhs, Self::Union(mut rhs)) => {
                if !rhs.contains(&lhs) {
                    rhs.insert(0, lhs);
                }
                rhs
            }
            (lhs, rhs) => vec![lhs, rhs],
        })
    }
}

/// A container for an argument.
pub trait Container {
    /// The contained type.
    type Inner;
}

impl<T> Container for Option<T> {
    type Inner = T;
}

impl<T> Container for Vec<T> {
    type Inner = T;
}

impl<T, const N: usize> Container for SmallVec<[T; N]>
where
    [T; N]: smallvec::Array,
{
    type Inner = T;
}

/// An uninhabitable type.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Never {}

impl Reflect for Never {
    fn input() -> CastInfo {
        CastInfo::Union(vec![])
    }

    fn output() -> CastInfo {
        CastInfo::Union(vec![])
    }

    fn castable(_: &TypstValue) -> bool {
        false
    }
}

impl IntoTypstValue for Never {
    fn into_value(self) -> TypstValue {
        match self {}
    }
}

impl FromTypstValue for Never {
    fn from_value(value: TypstValue) -> StrResult<Self> {
        Err(Self::error(&value))
    }
}

cast! {
    MathClass,
    self => IntoTypstValue::into_value(match self {
        MathClass::Normal => "normal",
        MathClass::Alphabetic => "alphabetic",
        MathClass::Binary => "binary",
        MathClass::Closing => "closing",
        MathClass::Diacritic => "diacritic",
        MathClass::Fence => "fence",
        MathClass::GlyphPart => "glyph-part",
        MathClass::Large => "large",
        MathClass::Opening => "opening",
        MathClass::Punctuation => "punctuation",
        MathClass::Relation => "relation",
        MathClass::Space => "space",
        MathClass::Unary => "unary",
        MathClass::Vary => "vary",
        MathClass::Special => "special",
    }),
    /// The default class for non-special things.
    "normal" => MathClass::Normal,
    /// Punctuation, e.g. a comma.
    "punctuation" => MathClass::Punctuation,
    /// An opening delimiter, e.g. `(`.
    "opening" => MathClass::Opening,
    /// A closing delimiter, e.g. `)`.
    "closing" => MathClass::Closing,
    /// A delimiter that is the same on both sides, e.g. `|`.
    "fence" => MathClass::Fence,
    /// A large operator like `sum`.
    "large" => MathClass::Large,
    /// A relation like `=` or `prec`.
    "relation" => MathClass::Relation,
    /// A unary operator like `not`.
    "unary" => MathClass::Unary,
    /// A binary operator like `times`.
    "binary" => MathClass::Binary,
    /// An operator that can be both unary or binary like `+`.
    "vary" => MathClass::Vary,
}
