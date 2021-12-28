#[macro_export]
macro_rules! repeat_less_than {
    ($n:expr) => {
        unsafe {
            static mut COUNTER: i32 = 0;
            if COUNTER > $n {
                panic!()
            } else {
                eprintln!("counter: {}", COUNTER);
                COUNTER += 1
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
