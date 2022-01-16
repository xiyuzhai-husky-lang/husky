#[macro_export]
macro_rules! repeat_less_than {
    ($n:expr) => {
        unsafe {
            static mut COUNTER: i32 = 0;
            if COUNTER > $n {
                panic!()
            } else {
                #[cfg(test)]
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
                    #[cfg(test)]
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
        match $result {
            true => (),
            false => {
                #[cfg(test)]
                println!("{}:{} should be true, but failed", file!(), line!());
                std::process::exit(1)
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
