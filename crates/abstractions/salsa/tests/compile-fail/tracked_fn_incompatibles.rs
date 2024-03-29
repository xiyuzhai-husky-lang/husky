#[salsa::jar]
struct Jar(
    MyInput,
    tracked_fn_with_data,
    tracked_fn_with_db,
    tracked_fn_with_constructor,
    tracked_fn_with_one_input,
    tracked_fn_with_receiver_not_applied_to_impl_block,
);

#[salsa::input(jar = Jar)]
struct MyInput {
    field: u32,
}

#[salsa::tracked(jar = Jar, data = Data)]
fn tracked_fn_with_data(db: &Db, input: MyInput) -> u32 {
    input.field(db) * 2
}

#[salsa::tracked(jar = Jar, db = Db)]
fn tracked_fn_with_db(db: &Db, input: MyInput) -> u32 {
    input.field(db) * 2
}

#[salsa::tracked(jar = Jar, constructor = TrackedFn3)]
fn tracked_fn_with_constructor(db: &Db, input: MyInput) -> u32 {
    input.field(db) * 2
}

#[salsa::tracked(jar = Jar)]
fn tracked_fn_with_one_input(db: &Db) -> u32 {}

#[salsa::tracked(jar = Jar)]
fn tracked_fn_with_receiver_not_applied_to_impl_block(&self, db: &Db) -> u32 {}

#[salsa::tracked(jar = Jar, specify)]
fn tracked_fn_with_receiver_not_applied_to_impl_block(
    db: &Db,
    input: MyInput,
    input: MyInput,
) -> u32 {
}

fn main() {}
