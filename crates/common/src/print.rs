#[macro_export]
macro_rules! p {
    ($($v:expr),*) => {
        assert_test_env();eprintln!(r#"
-------------------------------------------------------------------
{}{}:{}{}:{}
    {}"#,
        common::show::GREEN,
        file!(),
        common::show::YELLOW,
        line!(),
        common::show::RESET,
        show!($($v),*),
    )};
}

#[macro_export]
macro_rules! ep {
  ($($v:expr),*) => {eprintln!("src: {}:{}\n  {}\n",
  file!(),
  line!(),
  common::show::eshow!($($v),*))};
}

#[macro_export]
macro_rules! esp {
  ($($v:expr),*) => {eprintln!("src: {}:{}\n  {}\n",
  file!(),
  line!(),
  common::show::esimple_show!($($v),*))};
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
