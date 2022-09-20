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
        match (&$a, &$b) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    eprintln!(
                        r#"{}{}:{}{}{}
    {}{}{} (which is {:?}){}
        != {}{}{} (which is {:?}){}"#,
                        husky_print_utils::GREEN,
                        file!(),
                        husky_print_utils::YELLOW,
                        line!(),
                        husky_print_utils::RESET,
                        husky_print_utils::CYAN,
                        stringify!($a),
                        husky_print_utils::YELLOW,
                        &*left_val,
                        husky_print_utils::RED,
                        husky_print_utils::CYAN,
                        stringify!($b),
                        husky_print_utils::YELLOW,
                        &*right_val,
                        husky_print_utils::RESET,
                    );
                    panic!("");
                }
            }
        }
    };

    ($a:expr, $b:expr, $($msg_fmt_args: expr),*) => {
        match (&$a, &$b) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    eprintln!(
                        r#"{}{}:{}{}{}
    {}{}{} (which is {:?}){}
        != {}{}{} (which is {:?}){}"#,
                        husky_print_utils::GREEN,
                        file!(),
                        husky_print_utils::YELLOW,
                        line!(),
                        husky_print_utils::RESET,
                        husky_print_utils::CYAN,
                        stringify!($a),
                        husky_print_utils::YELLOW,
                        &*left_val,
                        husky_print_utils::RED,
                        husky_print_utils::CYAN,
                        stringify!($b),
                        husky_print_utils::YELLOW,
                        &*right_val,
                        husky_print_utils::RESET,
                    );
                    panic!($($msg_fmt_args),*);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! should_ok {
    ($result:expr) => {
        let result = $result;
        match result {
            Ok(v) => v,
            Err(should_ok_failed) => {
                p!(should_ok_failed);
                panic!()
            }
        }
    };
}

#[macro_export]
macro_rules! should {
    ($result:expr) => {{
        match $result {
            true => (),
            false => {
                panic!(
                    "{}{}{}:{} should be true, but failed",
                    husky_print_utils::GREEN,
                    file!(),
                    husky_print_utils::RESET,
                    line!()
                );
            }
        }
    }};

    ($result:expr, $($msg_fmt_args: expr),*) => {
        match $result {
            true => (),
            false => {
                panic!($($msg_fmt_args),*);
            }
        }
    };
}

#[macro_export]
macro_rules! should_satisfy {
    ($target: expr, $condition: expr) => {{
        if !$condition($target) {
            panic!(
                "expect for {:?} to satisfy {}, but failed",
                $target,
                stringify!($condition)
            );
        }
    }};
}
#[test]
fn test_repeat_less_than() {
    for _ in 0..1 {
        repeat_less_than!(10);
    }
}
