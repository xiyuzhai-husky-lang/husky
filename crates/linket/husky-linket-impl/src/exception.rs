use crate::{pedestal::IsPedestal, *};
use husky_ki_repr_interface::KiReprInterface;
use husky_value::{exception::IsException, ki_control_flow::KiControlFlow};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TrackedException<E: IsException, P: IsPedestal> {
    exception: E,
    source: ExceptionSource,
    pedestal: P,
}

impl<E: IsException, P: IsPedestal> TrackedException<E, P> {
    pub fn new(exception: E, source: ExceptionSource, pedestal: P) -> Self {
        Self {
            exception,
            source,
            pedestal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ExceptionSource {
    Ki(KiReprInterface),
}

impl<E: IsException, P: IsPedestal> TrackedException<E, P> {
    pub fn ki_catch_unwind<C, B>(
        f: impl FnOnce() -> C + std::panic::UnwindSafe,
    ) -> KiControlFlow<C, B, TrackedException<E, P>> {
        match std::panic::catch_unwind(f) {
            Ok(c) => KiControlFlow::Continue(c),
            Err(e) => match e.downcast::<TrackedException<E, P>>() {
                Ok(e) => KiControlFlow::Throw(*e),
                Err(e) => todo!(),
            },
        }
    }

    pub fn ki_catch_unwind2<C, B, R>(
        f: impl FnOnce() -> R + std::panic::UnwindSafe,
        convert: impl FnOnce(R) -> KiControlFlow<C, B, TrackedException<E, P>>,
    ) -> KiControlFlow<C, B, TrackedException<E, P>> {
        match std::panic::catch_unwind(f) {
            Ok(r) => convert(r),
            Err(e) => match e.downcast::<TrackedException<E, P>>() {
                Ok(e) => KiControlFlow::Throw(*e),
                Err(e) => todo!(),
            },
        }
    }
}

#[macro_export]
macro_rules! ki_catch_unwind {
    ($f: expr $(,)?) => {
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| $f()))
    };
    ($f: expr, $arg1: expr $(,)?) => {{
        let arg1 = $arg1;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| $f(arg1)))
    }};
    ($f: expr, $arg1: expr, $arg2: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| $f(arg1, arg2)))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3)))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11,
            )
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
            )
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
            )
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr, $arg14: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        let arg14 = $arg14;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
                arg14,
            )
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr, $arg14: expr, $arg15: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        let arg14 = $arg14;
        let arg15 = $arg15;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
                arg14, arg15,
            )
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr, $arg14: expr, $arg15: expr, $arg16: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        let arg14 = $arg14;
        let arg15 = $arg15;
        let arg16 = $arg16;
        TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
                arg14, arg15, arg16,
            )
        }))
    }};
}

#[macro_export]
macro_rules! ki_catch_unwind2 {
    ($f: expr, $convert: expr  $(,)?) => {
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f()), $convert)
    };
    ($f: expr, $convert: expr , $arg1: expr $(,)?) => {{
        let arg1 = $arg1;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3, arg4)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3, arg4, arg5)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3, arg4, arg5, arg6)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11,
            ),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
            ),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
            ),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr, $arg14: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        let arg14 = $arg14;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
                arg14,
            ),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr, $arg14: expr, $arg15: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        let arg14 = $arg14;
        let arg15 = $arg15;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
                arg14, arg15,
            ),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr, $arg14: expr, $arg15: expr, $arg16: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        let arg14 = $arg14;
        let arg15 = $arg15;
        let arg16 = $arg16;
        TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
                arg14, arg15, arg16,
            ),
            $convert
        }))
    }};
}

#[macro_export]
macro_rules! __ki_catch_unwind {
    ($f: expr $(,)?) => {
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| $f()))
    };
    ($f: expr, $arg1: expr $(,)?) => {{
        let arg1 = $arg1;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| $f(arg1)))
    }};
    ($f: expr, $arg1: expr, $arg2: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| $f(arg1, arg2)))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3)))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11,
            )
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
            )
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
            )
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr, $arg14: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        let arg14 = $arg14;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
                arg14,
            )
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr, $arg14: expr, $arg15: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        let arg14 = $arg14;
        let arg15 = $arg15;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
                arg14, arg15,
            )
        }))
    }};
    ($f: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr, $arg14: expr, $arg15: expr, $arg16: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        let arg14 = $arg14;
        let arg15 = $arg15;
        let arg16 = $arg16;
        __TrackedException::ki_catch_unwind(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
                arg14, arg15, arg16,
            )
        }))
    }};
}

#[macro_export]
macro_rules! __ki_catch_unwind2 {
    ($f: expr, $convert: expr  $(,)?) => {
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f()), $convert)
    };
    ($f: expr, $convert: expr , $arg1: expr $(,)?) => {{
        let arg1 = $arg1;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3, arg4)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3, arg4, arg5)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3, arg4, arg5, arg6)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)), $convert)
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11,
            ),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
            ),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
            ),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr, $arg14: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        let arg14 = $arg14;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
                arg14,
            ),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr, $arg14: expr, $arg15: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        let arg14 = $arg14;
        let arg15 = $arg15;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
                arg14, arg15,
            ),
            $convert
        }))
    }};
    ($f: expr, $convert: expr , $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr, $arg7: expr, $arg8: expr, $arg9: expr, $arg10: expr, $arg11: expr, $arg12: expr, $arg13: expr, $arg14: expr, $arg15: expr, $arg16: expr $(,)?) => {{
        let arg1 = $arg1;
        let arg2 = $arg2;
        let arg3 = $arg3;
        let arg4 = $arg4;
        let arg5 = $arg5;
        let arg6 = $arg6;
        let arg7 = $arg7;
        let arg8 = $arg8;
        let arg9 = $arg9;
        let arg10 = $arg10;
        let arg11 = $arg11;
        let arg12 = $arg12;
        let arg13 = $arg13;
        let arg14 = $arg14;
        let arg15 = $arg15;
        let arg16 = $arg16;
        __TrackedException::ki_catch_unwind2(std::panic::AssertUnwindSafe(|| {
            $f(
                arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
                arg14, arg15, arg16,
            ),
            $convert
        }))
    }};
}
