#[macro_export]
macro_rules! repeat_less_than {
    ($n:expr) => {
        #[cfg(debug_assertions)]
        unsafe {
            static mut COUNTER: i32 = 0;
            if COUNTER > $n {
                panic!()
            } else {
                eprintln!("{}:{}: counter: {}", file!(), line!(), COUNTER);
                COUNTER += 1
            }
        }
    };
}

#[macro_export]
macro_rules! should_eq {
    ($a:expr, $b:expr) => {
        #[cfg(debug_assertions)]
        match (&$a, &$b) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    eprintln!(
                        r#"{}{}:{}{}{}
    {}{}{} (which is {:?}){}
        != {}{}{} (which is {:?}){}"#,
                        common::show::GREEN,
                        file!(),
                        common::show::YELLOW,
                        line!(),
                        common::show::RESET,
                        common::show::CYAN,
                        stringify!($a),
                        common::show::YELLOW,
                        &*left_val,
                        common::show::RED,
                        common::show::CYAN,
                        stringify!($b),
                        common::show::YELLOW,
                        &*right_val,
                        common::show::RESET,
                    );
                    std::process::exit(1)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! should_ok {
    ($result:expr) => {
        match $result {
            Ok(v) => v,
            Err(should_ok_failed) => {
                p!(should_ok_failed);
                std::process::exit(1)
            }
        }
    };
}

#[macro_export]
macro_rules! should {
    ($result:expr) => {
        #[cfg(debug_assertions)]
        match $result {
            true => (),
            false => {
                panic!(
                    "{}{}{}:{} should be true, but failed",
                    common::show::GREEN,
                    file!(),
                    common::show::RESET,
                    line!()
                );
            }
        }
    };
}

#[test]
fn test_repeat_less_than() {
    for i in 0..1 {
        repeat_less_than!(10);
    }
}
