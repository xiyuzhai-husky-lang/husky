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
                        print_utils::GREEN,
                        file!(),
                        print_utils::YELLOW,
                        line!(),
                        print_utils::RESET,
                        print_utils::CYAN,
                        stringify!($a),
                        print_utils::YELLOW,
                        &*left_val,
                        print_utils::RED,
                        print_utils::CYAN,
                        stringify!($b),
                        print_utils::YELLOW,
                        &*right_val,
                        print_utils::RESET,
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
        let result = $result;
        #[cfg(debug_assertions)]
        match result {
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
    ($result:expr) => {{
        let result = $result;
        #[cfg(debug_assertions)]
        match result {
            true => (),
            false => {
                panic!(
                    "{}{}{}:{} should be true, but failed",
                    print_utils::GREEN,
                    file!(),
                    print_utils::RESET,
                    line!()
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! insert_new {
    ($map:expr, $key: expr, $value: expr) => {
        should!($map.insert($key, $value).is_none());
    };
}

#[test]
fn test_repeat_less_than() {
    for i in 0..1 {
        repeat_less_than!(10);
    }
}
