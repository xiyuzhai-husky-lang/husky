#[macro_export]
macro_rules! test_print {
    ($($v:expr),*) => {
        use husky_print_utils::*;
        #[cfg(test)]
        println!(r#"
{}
        {GREEN}{}:{YELLOW}{}:{RESET}"#,
        husky_print_utils::show!($($v),*),
        file!(),
        line!(),
    )};
}
#[macro_export]
macro_rules! p {
    ($($v:expr),* $(,)?) => {{
        if let Ok(_) = std::env::var("PRINT_COLORED") {
            eprintln!(
                    r#"{}
                --- {}{}:{}{}{}"#,
                $crate::show!($($v),*),
                $crate::GREEN,
                file!(),
                $crate::YELLOW,
                line!(),
                $crate::RESET,
            )
        } else {
            eprintln!(
                    r#"{}
                --- {}:{}"#,
                $crate::show!($($v),*),
                file!(),
                line!(),
            )
        }
    }};
}

#[macro_export]
macro_rules! ps {
    ($s:expr) => {
        format!(
            r#"{}{}:{}{}:{}
{}"#,
            husky_print_utils::GREEN,
            file!(),
            husky_print_utils::YELLOW,
            line!(),
            husky_print_utils::RESET,
            husky_print_utils::eshow!($s),
        )
    };
}

#[macro_export]
macro_rules! ep {
  ($($v:expr),*) => {eprintln!("src: {}:{}\n  {}\n",
  file!(),
  line!(),
  husky_print_utils::eshow!($($v),*))};
}

#[macro_export]
macro_rules! esp {
  ($($v:expr),*) => {eprintln!("src: {}:{}\n  {}\n",
  file!(),
  line!(),
  husky_print_utils::esimple_show!($($v),*))};
}

#[macro_export]
macro_rules! ep_once {
  ($($v:expr),*) => {common::do_once(||eprintln!("{}\n\t\tsrc: {}:{}",
  husky_print_utils::eshow!($($v),*),
  file!(),
  line!()))};
}

#[macro_export]
macro_rules! msg_once {
    ($msg:expr) => {{
        static ONCE: ::std::sync::Once = ::std::sync::Once::new();
        ONCE.call_once(|| eprintln!("[message] {}, src: {}:{}", $msg, file!(), line!()))
    }};
}

#[macro_export]
macro_rules! epin {
    () => {
        eprintln!("[pin] src: {}:{}", file!(), line!());
    };
}
