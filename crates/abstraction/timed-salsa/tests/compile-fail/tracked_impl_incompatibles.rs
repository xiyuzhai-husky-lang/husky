#[timed_salsa::jar(db = Db)]
struct Jar(MyTracked);

#[timed_salsa::tracked]
struct MyTracked {
    field: u32,
}

#[timed_salsa::tracked(return_ref)]
impl std::default::Default for MyTracked {
    fn default() -> Self {}
}
#[timed_salsa::tracked(specify)]
impl std::default::Default for MyTracked {
    fn default() -> Self {}
}

#[timed_salsa::tracked(no_eq)]
impl std::default::Default for MyTracked {
    fn default() -> Self {}
}

#[timed_salsa::tracked(data = Data)]
impl std::default::Default for MyTracked {
    fn default() -> Self {}
}

#[timed_salsa::tracked(db = Db)]
impl std::default::Default for MyTracked {
    fn default() -> Self {}
}

#[timed_salsa::tracked(recover_fn = recover)]
impl std::default::Default for MyTracked {
    fn default() -> Self {}
}

#[timed_salsa::tracked(lru = 32)]
impl std::default::Default for MyTracked {
    fn default() -> Self {}
}

#[timed_salsa::tracked(constructor = Constructor)]
impl std::default::Default for MyTracked {
    fn default() -> Self {}
}
#[timed_salsa::tracked]
impl std::default::Default for [MyTracked; 12] {
    fn default() -> Self {}
}

trait Db: timed_salsa::DbWithJar<Jar> {}

fn main() {}
