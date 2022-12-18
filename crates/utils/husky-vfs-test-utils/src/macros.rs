#[macro_export]
macro_rules! expect_test_crates {
    ($DB: ident, $method: ident) => {
        $DB::expect_test_crates(stringify!($method), $DB::$method)
    };
}

#[macro_export]
macro_rules! expect_test_modules {
    ($DB: ident, $method: ident) => {
        $DB::expect_test_modules(stringify!($method), $DB::$method)
    };
}

#[macro_export]
macro_rules! expect_test_entities {
    ($DB: ident, $method: ident) => {
        $DB::expect_test_entities(stringify!($method), $DB::$method)
    };
}
