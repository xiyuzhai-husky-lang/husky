pub extern crate colored;

/// if the value of `COUNTER` is positive when hanging, it indicates that the expr is the source of hanging
#[macro_export]
macro_rules! detect_hanging {
    ($expr: expr, $label: expr) => {{
        use $crate::colored::Colorize;

        thread_local! {
            static LABELS: std::cell::RefCell<Vec<String>> = Default::default();
        }
        LABELS.with_borrow_mut(|labels| labels.push($label));
        println!(
            "\n\n{} = {} at {}:{} before evaluation",
            "detect_hanging LABELS".red(),
            LABELS.with_borrow(|labels| format!("{labels:#?}")),
            file!(),
            line!()
        );
        let result = $expr;
        LABELS.with_borrow_mut(|labels| labels.pop());
        println!(
            "{} = {} at {}:{} after evaluation",
            "detect_hanging LABELS".green(),
            LABELS.with_borrow(|labels| format!("{labels:#?}")),
            file!(),
            line!()
        );
        result
    }};
}

#[macro_export]
macro_rules! detonate {
    ($count_down: expr) => {
        thread_local! {
            static COUNTER: std::cell::Cell<usize> = Default::default();
        }
        let counter = COUNTER.get();
        if counter >= $count_down {
            panic!("detonate! at {}:{}", file!(), line!())
        }
        COUNTER.set(counter + 1);
    };
    ($count_down: expr, $message: expr) => {
        thread_local! {
            static COUNTER: std::cell::Cell<usize> = Default::default();
        }
        let counter = COUNTER.get();
        if counter >= $count_down {
            panic!("detonate with `{}` at {}:{}!", $message, file!(), line!())
        }
        COUNTER.set(counter + 1);
    };
}
