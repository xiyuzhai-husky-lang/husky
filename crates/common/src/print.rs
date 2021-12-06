#[macro_export]
macro_rules! p {
  ($($v:expr),*) => {println!("{}{}:{}{}:{} {}",
  common::show::GREEN,
  file!(),
  common::show::YELLOW,
  line!(),
  common::show::RESET,
  show!($($v),*))};
}

#[macro_export]
macro_rules! ep {
  ($($v:expr),*) => {eprintln!("{}:{} {}",
  file!(),
  line!(),
  common::show::eshow!($($v),*))};
}
