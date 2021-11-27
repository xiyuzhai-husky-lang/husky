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
