// use crate::*;
// use syntax::*;

macro_rules! identify {
  ($phrase:expr, $arena:expr ) => {{
    $phrase.identify($arena)
  }};
}

macro_rules! unpair {
  ($pair:expr, $phrase_arena:expr) => {{
    $pair.unpair($phrase_arena, Join::List, file!(), line!())
  }};
}

macro_rules! uncurl {
  ($phrase:expr, $arena:expr, $f:expr) => {{
    ($phrase).unbracket_map($arena, Bracket::Curl, $f)
  }};
}
macro_rules! unbox {
  ($l:expr, $f:expr) => {{
    unbracket_map($l, Bracket::Box, $f)
  }};
}

macro_rules! unjoin {
  ($l:expr, $separator:expr, $f:expr, $message:expr) => {{
    throw_on_error!($l.unjoin_map($separator, $f), $message)
  }};
}

macro_rules! throw_on_error {
  ($result:expr,$message:expr) => {{
    push_call($result, file!(), line!(), $message)?
  }};
}

pub(crate) use identify;
pub(crate) use throw_on_error;
pub(crate) use unbox;
pub(crate) use uncurl;
pub(crate) use unjoin;
pub(crate) use unpair;
