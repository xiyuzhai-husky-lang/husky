#[timed_salsa::jar(db = Db, return_ref)]
struct JarWithRetRef(MyInput);

#[timed_salsa::jar(db = Db, specify)]
struct JarWithDb(MyInput);

#[timed_salsa::jar(db = Db, no_eq)]
struct JarWithNoEq(MyInput);

#[timed_salsa::jar(db = Db, jar = Jar)]
struct JarWithJar(MyInput);

#[timed_salsa::jar(db = Db, data = Data)]
struct JarWithData(MyInput);

#[timed_salsa::jar(db = Db, recovery_fn = recover)]
struct JarWithRecover(MyInput);

#[timed_salsa::jar(db = Db, lru = 32)]
struct JarWithLru(MyInput);

#[timed_salsa::jar(db = Db, constructor = JarConstructor)]
struct JarWithConstructor(MyInput);

#[timed_salsa::input(jar = Jar1)]
struct MyInput {
    field: u32,
}

fn main() {}
