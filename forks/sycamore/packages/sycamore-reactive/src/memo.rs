//! Derived and computed data.

use std::cell::Cell;

use husky_signal::Signalable;

use crate::*;

pub fn create_memo<'a, U: 'a>(
    cx: Scope<'a>,
    f: impl FnMut() -> U + 'a,
    info: String,
) -> &'a ReadSignal<U>
where
    U: Signalable,
{
    create_selector_with(cx, f, |_, _| false, info)
}

#[macro_export]
macro_rules! memo {
    ($visibility: ident, $lambda: expr) => {{
        create_memo($visibility, $lambda, format!("memo at {}:{}", file!(), line!()))
    }};
    ($visibility: ident, $lambda: expr, $first_dependency: ident) => {{
        let $first_dependency = $first_dependency.clone();
        memo!($visibility, $lambda)
    }};
    ($visibility: ident, $lambda: expr, $first_dependency: ident, $($other_dependencies: ident),*) => {{
        let $first_dependency = $first_dependency.clone();
        memo!($visibility, $lambda, $($other_dependencies),*)
    }};
}

#[macro_export]
macro_rules! selector {
    ($visibility: ident, $lambda: expr) => {{
        create_selector(
            $visibility,
            $lambda,
            format!("memo at {}:{}", file!(), line!()),
        )
    }};
}

pub fn create_selector<'a, U: PartialEq + 'a>(
    cx: Scope<'a>,
    f: impl FnMut() -> U + 'a,
    info: String,
) -> &'a ReadSignal<U>
where
    U: Signalable,
{
    create_selector_with(cx, f, PartialEq::eq, info)
}

pub fn create_selector_with<'a, U: 'a>(
    cx: Scope<'a>,
    mut f: impl FnMut() -> U + 'a,
    eq_f: impl Fn(&U, &U) -> bool + 'a,
    info: String,
) -> &'a ReadSignal<U>
where
    U: Signalable,
{
    let signal: &Cell<Option<&Signal<U>>> = create_ref(cx, Cell::new(None));

    create_effect(
        cx,
        move || {
            let new = f();
            if let Some(signal) = signal.get() {
                // Check if new value is different from old value.
                if !eq_f(&new, &*signal.get_untracked()) {
                    signal.set(new)
                }
            } else {
                signal.set(Some(create_signal(cx, new)))
            }
        },
        info,
    );

    signal.get().unwrap()
}

/// An alternative to [`create_signal`] that uses a reducer to get the next
/// value.
///
/// It uses a reducer function that takes the previous value and a message and returns the next
/// value.
///
/// Returns a [`ReadSignal`] and a dispatch function to send messages to the reducer.
///
/// # Params
/// * `initial` - The initial value of the state.
/// * `reducer` - A function that takes the previous value and a message and returns the next value.
///
/// # Example
/// ```
/// # use sycamore_reactive::*;
/// enum Msg {
///     Increment,
///     Decrement,
/// }
///
/// # create_scope_immediate(|cx| {
/// let (state, dispatch) = create_reducer(cx, 0, |state, msg: Msg| match msg {
///     Msg::Increment => *state + 1,
///     Msg::Decrement => *state - 1,
/// });
///
/// assert_eq!(*state.get(), 0);
/// dispatch(Msg::Increment);
/// assert_eq!(*state.get(), 1);
/// dispatch(Msg::Decrement);
/// assert_eq!(*state.get(), 0);
/// # });
/// ```
pub fn create_reducer<'a, U, Msg>(
    cx: Scope<'a>,
    initial: U,
    reduce: impl Fn(&U, Msg) -> U + 'a,
) -> (&'a ReadSignal<U>, impl Fn(Msg) + 'a)
where
    U: Signalable,
{
    let memo = create_signal(cx, initial);

    let dispatcher = move |msg| {
        memo.set(reduce(&memo.get_untracked(), msg));
    };

    (&*memo, dispatcher)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memo() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);
            let double = create_memo(
                cx,
                || *state.get() * 2,
                format!("src at {}:{}", file!(), line!()),
            );

            assert_eq!(*double.get(), 0);
            state.set(1);
            assert_eq!(*double.get(), 2);
            state.set(2);
            assert_eq!(*double.get(), 4);
        });
    }

    /// Make sure value is memoized rather than executed on demand.
    #[test]
    fn memo_only_run_once() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);

            let counter = create_signal(cx, 0);
            let double = create_memo(
                cx,
                || {
                    counter.set(*counter.get_untracked() + 1);
                    *state.get() * 2
                },
                format!("src at {}:{}", file!(), line!()),
            );

            assert_eq!(*counter.get(), 1); // once for calculating initial derived state
            state.set(2);
            assert_eq!(*counter.get(), 2);
            assert_eq!(*double.get(), 4);
            assert_eq!(*counter.get(), 2); // should still be 2 after access
        });
    }

    #[test]
    fn dependency_on_memo() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);
            let double = create_memo(
                cx,
                || *state.get() * 2,
                format!("src at {}:{}", file!(), line!()),
            );
            let quadruple = create_memo(
                cx,
                || *double.get() * 2,
                format!("src at {}:{}", file!(), line!()),
            );

            assert_eq!(*quadruple.get(), 0);
            state.set(1);
            assert_eq!(*quadruple.get(), 4);
        });
    }

    #[test]
    fn untracked_memo() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 1);
            let double = create_memo(
                cx,
                || *state.get_untracked() * 2,
                format!("src at {}:{}", file!(), line!()),
            );

            assert_eq!(*double.get(), 2);
            state.set(2);
            assert_eq!(*double.get(), 2); // double value should still be true because state.get()
                                          // was
                                          // inside untracked
        });
    }

    #[test]
    fn selector() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);
            let double = create_selector(
                cx,
                || *state.get() * 2,
                format!("src at {}:{}", file!(), line!()),
            );

            let counter = create_signal(cx, 0);
            effect!(cx, || {
                counter.set(*counter.get_untracked() + 1);

                double.track();
            });
            assert_eq!(*double.get(), 0);
            assert_eq!(*counter.get(), 1);

            state.set(0);
            assert_eq!(*double.get(), 0);
            assert_eq!(*counter.get(), 1); // calling set_state should not trigger the effect

            state.set(2);
            assert_eq!(*double.get(), 4);
            assert_eq!(*counter.get(), 2);
        });
    }

    #[test]
    fn reducer() {
        create_scope_immediate(|cx| {
            enum Msg {
                Increment,
                Decrement,
            }

            let (state, dispatch) = create_reducer(cx, 0, |state, msg: Msg| match msg {
                Msg::Increment => *state + 1,
                Msg::Decrement => *state - 1,
            });

            assert_eq!(*state.get(), 0);
            dispatch(Msg::Increment);
            assert_eq!(*state.get(), 1);
            dispatch(Msg::Decrement);
            assert_eq!(*state.get(), 0);
            dispatch(Msg::Increment);
            dispatch(Msg::Increment);
            assert_eq!(*state.get(), 2);
        });
    }

    #[test]
    fn memo_reducer() {
        create_scope_immediate(|cx| {
            enum Msg {
                Increment,
                Decrement,
            }

            let (state, dispatch) = create_reducer(cx, 0, |state, msg: Msg| match msg {
                Msg::Increment => *state + 1,
                Msg::Decrement => *state - 1,
            });
            let doubled = create_memo(
                cx,
                || *state.get() * 2,
                format!("src at {}:{}", file!(), line!()),
            );

            assert_eq!(*doubled.get(), 0);
            dispatch(Msg::Increment);
            assert_eq!(*doubled.get(), 2);
            dispatch(Msg::Decrement);
            assert_eq!(*doubled.get(), 0);
        });
    }
}
