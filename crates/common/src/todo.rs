#[macro_export]
macro_rules! td {
    () => {{
        // eprintln!("todo");
        set_hook_by_function!();
        panic!()
        // panic!("{}{}{}", RED, function!(), RESET)
    }};
}

#[macro_export]
macro_rules! set_hook_by_function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> String {
            std::any::type_name::<T>().to_string()
        }
        let name = type_name_of(f);
        std::panic::set_hook(Box::new(move |_| {
            println!(
                "{}{}:{}{}:{} {}todo {}{}{}",
                // file
                GREEN,
                file!(),
                // line
                YELLOW,
                line!(),
                column!(),
                // todo
                RED,
                // funcname
                RESET,
                &name[12..name.len() - 3],
                RESET,
            );
        }));
    }};
}

#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}
