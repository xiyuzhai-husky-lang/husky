use std::fmt::{self, Debug, Formatter};

use ecow::{eco_format, eco_vec, EcoString, EcoVec};

use crate::diag::{bail, error, At, SourceDiagnostic, TypstSourceResult};
use crate::foundations::{
    func, repr, scope, ty, Array, FromTypstValue, IntoTypstValue, Repr, Str, TypstDict, TypstValue,
};
use crate::syntax::{TypstSynSpan, TypstSynSpanned};

/// Captured arguments to a function.
///
/// # Argument Sinks
/// Like built-in functions, custom functions can also take a variable number of
/// arguments. You can specify an _argument sink_ which collects all excess
/// arguments as `..sink`. The resulting `sink` value is of the `arguments`
/// type. It exposes methods to access the positional and named arguments.
///
/// ```example
/// #let format(title, ..authors) = {
///   let by = authors
///     .pos()
///     .join(", ", last: " and ")
///
///   [*#title* \ _Written by #by;_]
/// }
///
/// #format("ArtosFlow", "Jane", "Joe")
/// ```
///
/// # Spreading
/// Inversely to an argument sink, you can _spread_ arguments, arrays and
/// dictionaries into a function call with the `..spread` operator:
///
/// ```example
/// #let array = (2, 3, 5)
/// #calc.min(..array)
/// #let dict = (fill: blue)
/// #text(..dict)[Hello]
/// ```
#[ty(scope, cast, name = "arguments")]
#[derive(Clone, Hash)]
#[allow(clippy::derived_hash_with_manual_eq)]
pub struct TypstArgs {
    /// The span of the whole argument list.
    pub span: TypstSynSpan,
    /// The positional and named arguments.
    pub items: EcoVec<TypstArg>,
}

impl TypstArgs {
    /// Create positional arguments from a span and values.
    pub fn new<T: IntoTypstValue>(span: TypstSynSpan, values: impl IntoIterator<Item = T>) -> Self {
        let items = values
            .into_iter()
            .map(|value| TypstArg {
                span,
                name: None,
                value: TypstSynSpanned::new(value.into_value(), span),
            })
            .collect();
        Self { span, items }
    }

    /// Returns the number of remaining positional arguments.
    pub fn remaining(&self) -> usize {
        self.items.iter().filter(|slot| slot.name.is_none()).count()
    }

    /// Push a positional argument.
    pub fn push(&mut self, span: TypstSynSpan, value: TypstValue) {
        self.items.push(TypstArg {
            span: self.span,
            name: None,
            value: TypstSynSpanned::new(value, span),
        })
    }

    /// Consume and cast the first positional argument if there is one.
    pub fn eat<T>(&mut self) -> TypstSourceResult<Option<T>>
    where
        T: FromTypstValue<TypstSynSpanned<TypstValue>>,
    {
        for (i, slot) in self.items.iter().enumerate() {
            if slot.name.is_none() {
                let value = self.items.remove(i).value;
                let span = value.span;
                return T::from_value(value).at(span).map(Some);
            }
        }
        Ok(None)
    }

    /// Consume n positional arguments if possible.
    pub fn consume(&mut self, n: usize) -> TypstSourceResult<Vec<TypstArg>> {
        let mut list = vec![];

        let mut i = 0;
        while i < self.items.len() && list.len() < n {
            if self.items[i].name.is_none() {
                list.push(self.items.remove(i));
            } else {
                i += 1;
            }
        }

        if list.len() < n {
            bail!(self.span, "not enough arguments");
        }

        Ok(list)
    }

    /// Consume and cast the first positional argument.
    ///
    /// Returns a `missing argument: {what}` error if no positional argument is
    /// left.
    pub fn expect<T>(&mut self, what: &str) -> TypstSourceResult<T>
    where
        T: FromTypstValue<TypstSynSpanned<TypstValue>>,
    {
        match self.eat()? {
            Some(v) => Ok(v),
            None => bail!(self.missing_argument(what)),
        }
    }

    /// The error message for missing arguments.
    fn missing_argument(&self, what: &str) -> SourceDiagnostic {
        for item in &self.items {
            let Some(name) = item.name.as_deref() else {
                continue;
            };
            if name == what {
                return error!(
                    item.span,
                    "the argument `{what}` is positional";
                    hint: "try removing `{}:`", name,
                );
            }
        }

        error!(self.span, "missing argument: {what}")
    }

    /// Find and consume the first castable positional argument.
    pub fn find<T>(&mut self) -> TypstSourceResult<Option<T>>
    where
        T: FromTypstValue<TypstSynSpanned<TypstValue>>,
    {
        for (i, slot) in self.items.iter().enumerate() {
            if slot.name.is_none() && T::castable(&slot.value.v) {
                let value = self.items.remove(i).value;
                let span = value.span;
                return T::from_value(value).at(span).map(Some);
            }
        }
        Ok(None)
    }

    /// Find and consume all castable positional arguments.
    pub fn all<T>(&mut self) -> TypstSourceResult<Vec<T>>
    where
        T: FromTypstValue<TypstSynSpanned<TypstValue>>,
    {
        let mut list = vec![];
        let mut errors = eco_vec![];
        self.items.retain(|item| {
            if item.name.is_some() {
                return true;
            };
            let span = item.value.span;
            let spanned = TypstSynSpanned::new(std::mem::take(&mut item.value.v), span);
            match T::from_value(spanned) {
                Ok(val) => list.push(val),
                Err(err) => errors.push(SourceDiagnostic::error(span, err)),
            }
            false
        });
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(list)
    }

    /// Cast and remove the value for the given named argument, returning an
    /// error if the conversion fails.
    pub fn named<T>(&mut self, name: &str) -> TypstSourceResult<Option<T>>
    where
        T: FromTypstValue<TypstSynSpanned<TypstValue>>,
    {
        // We don't quit once we have a match because when multiple matches
        // exist, we want to remove all of them and use the last one.
        let mut i = 0;
        let mut found = None;
        while i < self.items.len() {
            if self.items[i].name.as_deref() == Some(name) {
                let value = self.items.remove(i).value;
                let span = value.span;
                found = Some(T::from_value(value).at(span)?);
            } else {
                i += 1;
            }
        }
        Ok(found)
    }

    /// Same as named, but with fallback to find.
    pub fn named_or_find<T>(&mut self, name: &str) -> TypstSourceResult<Option<T>>
    where
        T: FromTypstValue<TypstSynSpanned<TypstValue>>,
    {
        match self.named(name)? {
            Some(value) => Ok(Some(value)),
            None => self.find(),
        }
    }

    /// Take out all arguments into a new instance.
    pub fn take(&mut self) -> Self {
        Self {
            span: self.span,
            items: std::mem::take(&mut self.items),
        }
    }

    /// Return an "unexpected argument" error if there is any remaining
    /// argument.
    pub fn finish(self) -> TypstSourceResult<()> {
        if let Some(arg) = self.items.first() {
            match &arg.name {
                Some(name) => bail!(arg.span, "unexpected argument: {name}"),
                _ => bail!(arg.span, "unexpected argument"),
            }
        }
        Ok(())
    }
}

#[scope]
impl TypstArgs {
    /// Construct spreadable arguments in place.
    ///
    /// This function behaves like `{let args(..sink) = sink}`.
    ///
    /// ```example
    /// #let args = arguments(stroke: red, inset: 1em, [Body])
    /// #box(..args)
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// The real arguments (the other argument is just for the docs).
        /// The docs argument cannot be called `args`.
        args: &mut TypstArgs,
        /// The arguments to construct.
        #[external]
        #[variadic]
        arguments: Vec<TypstValue>,
    ) -> TypstArgs {
        args.take()
    }

    /// Returns the captured positional arguments as an array.
    #[func(name = "pos", title = "Positional")]
    pub fn to_pos(&self) -> Array {
        self.items
            .iter()
            .filter(|item| item.name.is_none())
            .map(|item| item.value.v.clone())
            .collect()
    }

    /// Returns the captured named arguments as a dictionary.
    #[func(name = "named")]
    pub fn to_named(&self) -> TypstDict {
        self.items
            .iter()
            .filter_map(|item| item.name.clone().map(|name| (name, item.value.v.clone())))
            .collect()
    }
}

impl Debug for TypstArgs {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_list().entries(&self.items).finish()
    }
}

impl Repr for TypstArgs {
    fn repr(&self) -> EcoString {
        let pieces = self.items.iter().map(TypstArg::repr).collect::<Vec<_>>();
        repr::pretty_array_like(&pieces, false).into()
    }
}

impl PartialEq for TypstArgs {
    fn eq(&self, other: &Self) -> bool {
        self.to_pos() == other.to_pos() && self.to_named() == other.to_named()
    }
}

/// An argument to a function call: `12` or `draw: false`.
#[derive(Clone, Hash)]
#[allow(clippy::derived_hash_with_manual_eq)]
pub struct TypstArg {
    /// The span of the whole argument.
    pub span: TypstSynSpan,
    /// The name of the argument (`None` for positional arguments).
    pub name: Option<Str>,
    /// The value of the argument.
    pub value: TypstSynSpanned<TypstValue>,
}

impl Debug for TypstArg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(name) = &self.name {
            name.fmt(f)?;
            f.write_str(": ")?;
            self.value.v.fmt(f)
        } else {
            self.value.v.fmt(f)
        }
    }
}

impl Repr for TypstArg {
    fn repr(&self) -> EcoString {
        if let Some(name) = &self.name {
            eco_format!("{}: {}", name, self.value.v.repr())
        } else {
            self.value.v.repr()
        }
    }
}

impl PartialEq for TypstArg {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value.v == other.value.v
    }
}

/// Things that can be used as arguments.
pub trait IntoArgs {
    /// Convert into arguments, attaching the `fallback` span in case `Self`
    /// doesn't have a span.
    fn into_args(self, fallback: TypstSynSpan) -> TypstArgs;
}

impl IntoArgs for TypstArgs {
    fn into_args(self, _: TypstSynSpan) -> TypstArgs {
        self
    }
}

impl<I, T> IntoArgs for I
where
    I: IntoIterator<Item = T>,
    T: IntoTypstValue,
{
    fn into_args(self, fallback: TypstSynSpan) -> TypstArgs {
        TypstArgs::new(fallback, self)
    }
}
