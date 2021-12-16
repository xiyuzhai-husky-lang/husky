#[macro_export]
macro_rules! p {
  ($($v:expr),*) => {
    assert_test_env();eprintln!("\n-------------------------------------------------------------------\n{}\n    src: {}{}:{}{}.{}\n",
    show!($($v),*),
    common::show::GREEN,
    file!(),
    common::show::YELLOW,
    line!(),
    common::show::RESET
  )};
}

#[macro_export]
macro_rules! ep {
  ($($v:expr),*) => {eprintln!("{}\n    src: {}:{}",
  common::show::eshow!($($v),*),
  file!(),
  line!())};
}

#[macro_export]
macro_rules! ep_once {
  ($($v:expr),*) => {common::do_once(||eprintln!("{}\n\t\tsrc: {}:{}",
  common::show::eshow!($($v),*),
  file!(),
  line!()))};
}

#[macro_export]
macro_rules! msg_once {
    ($msg:expr) => {
        common::do_once(|| eprintln!("[message] {}\n\t\tsrc: {}:{}", $msg, file!(), line!()))
    };
}

#[macro_export]
macro_rules! epin {
    () => {
        eprintln!("[pin] src: {}:{}.", file!(), line!());
    };
}
