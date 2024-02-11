use std::borrow::Cow;
use std::fmt::Write;
use std::hash::Hash;
use std::ops::Add;

use comemo::Prehashed;
use ecow::{eco_format, EcoString};
use smallvec::SmallVec;
use unicode_math_class::MathClass;

use crate::diag::{At, SourceResult, StrResult};
use crate::foundations::{array, repr, Packed, Repr, Str, TexElement, TexValue, Type};
use crate::syntax::{Span, Spanned};

#[rustfmt::skip]
#[doc(inline)]
pub use husky_tex_macros::{cast, Cast};

/// Determine details of a type.
///
/// Type casting works as follows:
/// - [`Reflect for T`](Reflect) describes the possible Tex values for `T`
///    (for documentation and autocomplete).
/// - [`IntoTexValue for T`](IntoTexValue) is for conversion from `T -> TexValue`
///   (infallible)
/// - [`FromTexValue for T`](FromTexValue) is for conversion from `TexValue -> T`
///   (fallible).
///
/// We can't use `TryFrom<TexValue>` due to conflicting impls. We could use
/// `From<T> for TexValue`, but that inverses the impl and leads to tons of
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
    fn castable(value: &TexValue) -> bool;

    /// Produce an error message for an inacceptable value type.
    ///
    /// ```
    /// assert_eq!(
    ///   <Int as Reflect>::error(&TexValue::None),
    ///   "expected integer, found none",
    /// );
    /// ```
    fn error(found: &TexValue) -> EcoString {
        Self::input().error(found)
    }
}

impl Reflect for TexValue {
    fn input() -> CastInfo {
        CastInfo::Any
    }

    fn output() -> CastInfo {
        CastInfo::Any
    }

    fn castable(_: &TexValue) -> bool {
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

    fn castable(value: &TexValue) -> bool {
        T::castable(value)
    }
}

impl<T: TexElement + Reflect> Reflect for Packed<T> {
    fn input() -> CastInfo {
        T::input()
    }

    fn output() -> CastInfo {
        T::output()
    }

    fn castable(value: &TexValue) -> bool {
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

    fn castable(value: &TexValue) -> bool {
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

    fn castable(value: &TexValue) -> bool {
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

    fn castable(value: &TexValue) -> bool {
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

    fn castable(value: &TexValue) -> bool {
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

    fn castable(value: &TexValue) -> bool {
        T::castable(value)
    }
}

/// Cast a Rust type into a Tex [`TexValue`].
///
/// See also: [`Reflect`].
pub trait IntoTexValue {
    /// Cast this type into a value.
    fn into_value(self) -> TexValue;
}

impl IntoTexValue for TexValue {
    fn into_value(self) -> TexValue {
        self
    }
}

impl IntoTexValue for (&Str, &TexValue) {
    fn into_value(self) -> TexValue {
        TexValue::Array(array![self.0.clone(), self.1.clone()])
    }
}

impl<T: IntoTexValue + Clone> IntoTexValue for Cow<'_, T> {
    fn into_value(self) -> TexValue {
        self.into_owned().into_value()
    }
}

impl<T: TexElement + IntoTexValue> IntoTexValue for Packed<T> {
    fn into_value(self) -> TexValue {
        TexValue::Content(self.pack())
    }
}

impl<T: IntoTexValue> IntoTexValue for Spanned<T> {
    fn into_value(self) -> TexValue {
        self.v.into_value()
    }
}

impl<T: IntoTexValue + Hash + 'static> IntoTexValue for Prehashed<T> {
    fn into_value(self) -> TexValue {
        self.into_inner().into_value()
    }
}

/// Cast a Rust type or result into a [`SourceResult<TexValue>`].
///
/// Converts `T`, [`StrResult<T>`], or [`SourceResult<T>`] into
/// [`SourceResult<TexValue>`] by `Ok`-wrapping or adding span information.
pub trait IntoResult {
    /// Cast this type into a value.
    fn into_result(self, span: Span) -> SourceResult<TexValue>;
}

impl<T: IntoTexValue> IntoResult for T {
    fn into_result(self, _: Span) -> SourceResult<TexValue> {
        Ok(self.into_value())
    }
}

impl<T: IntoTexValue> IntoResult for StrResult<T> {
    fn into_result(self, span: Span) -> SourceResult<TexValue> {
        self.map(IntoTexValue::into_value).at(span)
    }
}

impl<T: IntoTexValue> IntoResult for SourceResult<T> {
    fn into_result(self, _: Span) -> SourceResult<TexValue> {
        self.map(IntoTexValue::into_value)
    }
}

impl<T: IntoTexValue> IntoTexValue for fn() -> T {
    fn into_value(self) -> TexValue {
        self().into_value()
    }
}

/// Try to cast a Tex [`TexValue`] into a Rust type.
///
/// See also: [`Reflect`].
pub trait FromTexValue<V = TexValue>: Sized + Reflect {
    /// Try to cast the value into an instance of `Self`.
    fn from_value(value: V) -> StrResult<Self>;
}

impl FromTexValue for TexValue {
    fn from_value(value: TexValue) -> StrResult<Self> {
        Ok(value)
    }
}

impl<T: TexElement + FromTexValue> FromTexValue for Packed<T> {
    fn from_value(mut value: TexValue) -> StrResult<Self> {
        if let TexValue::Content(content) = value {
            match content.into_packed::<T>() {
                Ok(packed) => return Ok(packed),
                Err(content) => value = TexValue::Content(content),
            }
        }
        let val = T::from_value(value)?;
        Ok(Packed::new(val))
    }
}

impl<T: FromTexValue + Hash + 'static> FromTexValue for Prehashed<T> {
    fn from_value(value: TexValue) -> StrResult<Self> {
        Ok(Self::new(T::from_value(value)?))
    }
}

impl<T: FromTexValue> FromTexValue<Spanned<TexValue>> for T {
    fn from_value(value: Spanned<TexValue>) -> StrResult<Self> {
        T::from_value(value.v)
    }
}

impl<T: FromTexValue> FromTexValue<Spanned<TexValue>> for Spanned<T> {
    fn from_value(value: Spanned<TexValue>) -> StrResult<Self> {
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
    TexValue(TexValue, &'static str),
    /// Any value of a type.
    Type(Type),
    /// Multiple alternatives.
    Union(Vec<Self>),
}

impl CastInfo {
    /// Produce an error message describing what was expected and what was
    /// found.
    pub fn error(&self, found: &TexValue) -> EcoString {
        let mut matching_type = false;
        let mut parts = vec![];

        self.walk(|info| match info {
            CastInfo::Any => parts.push("anything".into()),
            CastInfo::TexValue(value, _) => {
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

        if let TexValue::Int(i) = found {
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

    fn castable(_: &TexValue) -> bool {
        false
    }
}

impl IntoTexValue for Never {
    fn into_value(self) -> TexValue {
        match self {}
    }
}

impl FromTexValue for Never {
    fn from_value(value: TexValue) -> StrResult<Self> {
        Err(Self::error(&value))
    }
}

cast! {
    MathClass,
    self => IntoTexValue::into_value(match self {
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
