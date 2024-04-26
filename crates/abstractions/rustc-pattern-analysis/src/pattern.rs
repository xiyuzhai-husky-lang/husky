//! As explained in [`crate::usefulness`], values and patterns are made from constructors applied to
//! fields. This file defines types that represent patterns in this way.
use self::Constructor::*;
use crate::constructor::{Constructor, Slice, SliceKind};
use crate::{IsPatternAnalyisContext, PrivateUninhabitedField};
use smallvec::{smallvec, SmallVec};
use std::fmt;

/// A globally unique id to distinguish patterns.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PatId(u32);
impl PatId {
    fn new() -> Self {
        use std::sync::atomic::{AtomicU32, Ordering};
        static PAT_ID: AtomicU32 = AtomicU32::new(0);
        PatId(PAT_ID.fetch_add(1, Ordering::SeqCst))
    }
}

/// A pattern with an index denoting which field it corresponds to.
pub struct IndexedPat<Ctx: IsPatternAnalyisContext> {
    pub idx: usize,
    pub pat: DeconstructedPattern<Ctx>,
}

/// Values and patterns can be represented as a constructor applied to some fields. This represents
/// a pattern in this form. A `DeconstructedPattern` will almost always come from user input; the only
/// exception are some `Wildcard`s introduced during pattern lowering.
pub struct DeconstructedPattern<Ctx: IsPatternAnalyisContext> {
    constructor: Constructor<Ctx>,
    fields: Vec<IndexedPat<Ctx>>,
    /// The number of fields in this pattern. E.g. if the pattern is `SomeStruct { field12: true, ..
    /// }` this would be the total number of fields of the struct.
    /// This is also the same as `self.constructor.arity(self.ty)`.
    arity: usize,
    ty: Ctx::Type,
    /// Extra data to store in a pattern.
    data: Ctx::PatternDataExtra,
    /// Globally-unique id used to track usefulness at the level of subpatterns.
    pub(crate) uid: PatId,
}

impl<Ctx: IsPatternAnalyisContext> DeconstructedPattern<Ctx> {
    pub fn new(
        constructor: Constructor<Ctx>,
        fields: Vec<IndexedPat<Ctx>>,
        arity: usize,
        ty: Ctx::Type,
        data: Ctx::PatternDataExtra,
    ) -> Self {
        DeconstructedPattern {
            constructor,
            fields,
            arity,
            ty,
            data,
            uid: PatId::new(),
        }
    }

    pub fn at_index(self, idx: usize) -> IndexedPat<Ctx> {
        IndexedPat { idx, pat: self }
    }

    pub(crate) fn is_or_pat(&self) -> bool {
        matches!(self.constructor, Or)
    }

    pub fn constructor(&self) -> &Constructor<Ctx> {
        &self.constructor
    }
    pub fn ty(&self) -> &Ctx::Type {
        &self.ty
    }
    /// Returns the extra data stored in a pattern.
    pub fn data(&self) -> &Ctx::PatternDataExtra {
        &self.data
    }
    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn iter_fields<'a>(&'a self) -> impl Iterator<Item = &'a IndexedPat<Ctx>> {
        self.fields.iter()
    }

    /// Specialize this pattern with a constructor.
    /// `other_ctor` can be different from `self.constructor`, but must be covered by it.
    pub(crate) fn specialize<'a>(
        &'a self,
        other_ctor: &Constructor<Ctx>,
        other_constructor_arity: usize,
    ) -> SmallVec<[UserPatternOrDerivedWildcard<'a, Ctx>; 2]> {
        if matches!(other_ctor, PrivateUninhabited) {
            // Skip this column.
            return smallvec![];
        }

        // Start with a slice of wildcards of the appropriate length.
        let mut fields: SmallVec<[_; 2]> = (0..other_constructor_arity)
            .map(|_| UserPatternOrDerivedWildcard::DerivedWildcard)
            .collect();
        // Fill `fields` with our fields. The arities are known to be compatible.
        match self.constructor {
            // The only non-trivial case: two slices of different arity. `other_ctor` is guaranteed
            // to have a larger arity, so we adjust the indices of the patterns in the suffix so
            // that they are correctly positioned in the larger slice.
            Slice(Slice {
                kind: SliceKind::VarLen(prefix, _),
                ..
            }) if self.arity != other_constructor_arity => {
                for ipat in &self.fields {
                    let new_idx = if ipat.idx < prefix {
                        ipat.idx
                    } else {
                        // Adjust the indices in the suffix.
                        ipat.idx + other_constructor_arity - self.arity
                    };
                    fields[new_idx] = UserPatternOrDerivedWildcard::UserPattern(&ipat.pat);
                }
            }
            _ => {
                for ipat in &self.fields {
                    fields[ipat.idx] = UserPatternOrDerivedWildcard::UserPattern(&ipat.pat);
                }
            }
        }
        fields
    }

    /// Walk top-down and call `it` in each place where a pattern occurs
    /// starting with the root pattern `walk` is called on. If `it` returns
    /// false then we will descend no further but siblings will be processed.
    pub fn walk<'a>(&'a self, it: &mut impl FnMut(&'a Self) -> bool) {
        if !it(self) {
            return;
        }

        for p in self.iter_fields() {
            p.pat.walk(it)
        }
    }
}

/// This is best effort and not good enough for a `Display` impl.
impl<Ctx: IsPatternAnalyisContext> fmt::Debug for DeconstructedPattern<Ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fields: Vec<_> = (0..self.arity)
            .map(|_| UserPatternOrDerivedWildcard::DerivedWildcard)
            .collect();
        for ipat in self.iter_fields() {
            fields[ipat.idx] = UserPatternOrDerivedWildcard::UserPattern(&ipat.pat);
        }
        self.constructor()
            .fmt_fields(f, self.ty(), fields.into_iter())
    }
}

/// Represents either a pattern obtained from user input or a wildcard constructed during the
/// algorithm. Do not use `Wild` to represent a wildcard pattern comping from user input.
///
/// This is morally `Option<&'p DeconstructedPattern>` where `None` is interpreted as a wildcard.
pub(crate) enum UserPatternOrDerivedWildcard<'p, Ctx: IsPatternAnalyisContext> {
    /// A non-user-provided wildcard, created during specialization.
    DerivedWildcard,
    /// A user-provided pattern.
    UserPattern(&'p DeconstructedPattern<Ctx>),
}

impl<'p, Ctx: IsPatternAnalyisContext> Clone for UserPatternOrDerivedWildcard<'p, Ctx> {
    fn clone(&self) -> Self {
        match self {
            UserPatternOrDerivedWildcard::DerivedWildcard => {
                UserPatternOrDerivedWildcard::DerivedWildcard
            }
            UserPatternOrDerivedWildcard::UserPattern(pat) => {
                UserPatternOrDerivedWildcard::UserPattern(pat)
            }
        }
    }
}

impl<'p, Ctx: IsPatternAnalyisContext> Copy for UserPatternOrDerivedWildcard<'p, Ctx> {}

impl<'p, Ctx: IsPatternAnalyisContext> UserPatternOrDerivedWildcard<'p, Ctx> {
    pub(crate) fn as_pat(&self) -> Option<&'p DeconstructedPattern<Ctx>> {
        match self {
            UserPatternOrDerivedWildcard::DerivedWildcard => None,
            UserPatternOrDerivedWildcard::UserPattern(pat) => Some(pat),
        }
    }
    pub(crate) fn constructor(self) -> &'p Constructor<Ctx> {
        match self {
            UserPatternOrDerivedWildcard::DerivedWildcard => &Wildcard,
            UserPatternOrDerivedWildcard::UserPattern(pat) => pat.constructor(),
        }
    }

    pub(crate) fn is_or_pat(&self) -> bool {
        match self {
            UserPatternOrDerivedWildcard::DerivedWildcard => false,
            UserPatternOrDerivedWildcard::UserPattern(pat) => pat.is_or_pat(),
        }
    }

    /// Expand this (possibly-nested) or-pattern into its alternatives.
    pub(crate) fn flatten_or_pat(self) -> SmallVec<[Self; 1]> {
        match self {
            UserPatternOrDerivedWildcard::UserPattern(pat) if pat.is_or_pat() => pat
                .iter_fields()
                .flat_map(|ipat| {
                    UserPatternOrDerivedWildcard::UserPattern(&ipat.pat).flatten_or_pat()
                })
                .collect(),
            _ => smallvec![self],
        }
    }

    /// Specialize this pattern with a constructor.
    /// `other_ctor` can be different from `self.constructor`, but must be covered by it.
    pub(crate) fn specialize(
        &self,
        other_ctor: &Constructor<Ctx>,
        constructor_arity: usize,
    ) -> SmallVec<[UserPatternOrDerivedWildcard<'p, Ctx>; 2]> {
        match self {
            UserPatternOrDerivedWildcard::DerivedWildcard => (0..constructor_arity)
                .map(|_| UserPatternOrDerivedWildcard::DerivedWildcard)
                .collect(),
            UserPatternOrDerivedWildcard::UserPattern(pat) => {
                pat.specialize(other_ctor, constructor_arity)
            }
        }
    }
}

impl<'p, Ctx: IsPatternAnalyisContext> fmt::Debug for UserPatternOrDerivedWildcard<'p, Ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserPatternOrDerivedWildcard::DerivedWildcard => write!(f, "_"),
            UserPatternOrDerivedWildcard::UserPattern(pat) => pat.fmt(f),
        }
    }
}

/// Same idea as `DeconstructedPattern`, except this is a fictitious pattern built up for diagnostics
/// purposes. As such they don't use interning and can be cloned.
pub struct WitnessPattern<Ctx: IsPatternAnalyisContext> {
    constructor: Constructor<Ctx>,
    pub(crate) fields: Vec<WitnessPattern<Ctx>>,
    ty: Ctx::Type,
}

impl<Ctx: IsPatternAnalyisContext> Clone for WitnessPattern<Ctx> {
    fn clone(&self) -> Self {
        Self {
            constructor: self.constructor.clone(),
            fields: self.fields.clone(),
            ty: self.ty.clone(),
        }
    }
}

impl<Ctx: IsPatternAnalyisContext> WitnessPattern<Ctx> {
    pub(crate) fn new(constructor: Constructor<Ctx>, fields: Vec<Self>, ty: Ctx::Type) -> Self {
        Self {
            constructor,
            fields,
            ty,
        }
    }
    /// Create a wildcard pattern for this type. If the type is empty, we create a `!` pattern.
    pub(crate) fn wildcard(ctx: &Ctx, ty: Ctx::Type) -> Self {
        let is_empty = ctx
            .constructors_for_ty(&ty)
            .is_ok_and(|ctors| ctors.all_empty());
        let constructor = if is_empty { Never } else { Wildcard };
        Self::new(constructor, Vec::new(), ty)
    }

    /// Construct a pattern that matches everything that starts with this constructor.
    /// For example, if `constructor` is a `Constructor::Variant` for `Option::Some`, we get the pattern
    /// `Some(_)`.
    pub(crate) fn derive_wildcard_from_constructor(
        ctx: &Ctx,
        constructor: Constructor<Ctx>,
        ty: Ctx::Type,
    ) -> Self {
        if matches!(constructor, Wildcard) {
            return Self::wildcard(ctx, ty);
        }
        let fields = ctx
            .constructor_field_tys(&constructor, &ty)
            .filter(|(_, PrivateUninhabitedField(skip))| !skip)
            .map(|(ty, _)| Self::wildcard(ctx, ty))
            .collect();
        Self::new(constructor, fields, ty)
    }

    pub fn constructor(&self) -> &Constructor<Ctx> {
        &self.constructor
    }
    pub fn ty(&self) -> &Ctx::Type {
        &self.ty
    }

    pub fn is_never_pattern(&self) -> bool {
        match self.constructor() {
            Never => true,
            Or => self.fields.iter().all(|p| p.is_never_pattern()),
            _ => self.fields.iter().any(|p| p.is_never_pattern()),
        }
    }

    pub fn iter_fields(&self) -> impl Iterator<Item = &WitnessPattern<Ctx>> {
        self.fields.iter()
    }
}

/// This is best effort and not good enough for a `Display` impl.
impl<Ctx: IsPatternAnalyisContext> fmt::Debug for WitnessPattern<Ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.constructor()
            .fmt_fields(f, self.ty(), self.fields.iter())
    }
}
