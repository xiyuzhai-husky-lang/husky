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
