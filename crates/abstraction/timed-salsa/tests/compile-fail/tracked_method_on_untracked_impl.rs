#[timed_salsa::jar(db = Db)]
struct Jar(MyInput, tracked_method_on_untracked_impl);

trait Db: timed_salsa::DbWithJar<Jar> {}

#[timed_salsa::input]
struct MyInput {
    field: u32,
}

impl MyInput {
    #[timed_salsa::tracked]
    fn tracked_method_on_untracked_impl(self, db: &dyn Db) -> u32 {
        input.field(db)
    }
}

fn main() {}
