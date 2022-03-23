use std::fmt::*;
#[macro_export]
macro_rules! show {
    ($a:expr)=>{format!("{}{}{} = {:#?}",
    print_utils::PINK,stringify!($a),
    print_utils::RESET,
    $a)};
   ($a:expr ,$($as:expr),*) => {
    format!("{}, {}", (print_utils::show!($a)), (print_utils::show!($($as),*)))
  };
}

#[macro_export]
macro_rules! eshow {
    ($a:expr)=>{format!("{} = {:#?}",
    stringify!($a),
    $a)};
   ($a:expr, $($as:expr),*) => {
    format!("{}, {}", (print_utils::show!($a)), (print_utils::show!($($as),*)))
  };
}

#[macro_export]
macro_rules! esimple_show {
    ($a:expr)=>{format!("{} = {:?}",
    stringify!($a),
    $a)};
   ($a:expr, $($as:expr),*) => {
    format!("{}, {}", (print_utils::show!($a)), (print_utils::show!($($as),*)))
  };
}

pub const RESET: &str = "\x1B[0m";
pub const BLACK: &str = "\x1B[30m";
pub const RED: &str = "\x1B[31m";
pub const GREEN: &str = "\x1B[32m";
pub const YELLOW: &str = "\x1B[38;2;220;180;55m";
pub const BLUE: &str = "\x1B[34m";
pub const MAGENTA: &str = "\x1B[35m";
pub const CYAN: &str = "\x1B[36m";
pub const WHITE: &str = "\x1B[37m";
pub const BOLDBLACK: &str = "\033[1m\033[30m";
pub const PINK: &str = "\x1B[38;2;250;50;155m";

pub struct ShowCase {
    content: String,
}
impl ShowCase {
    pub fn new(content: String) -> ShowCase {
        ShowCase { content }
    }
}
impl std::fmt::Debug for ShowCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{}", self.content)
    }
}

pub use eshow;
pub use esimple_show;
pub use show;
