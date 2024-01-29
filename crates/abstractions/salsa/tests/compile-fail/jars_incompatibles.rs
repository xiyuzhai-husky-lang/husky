#[salsa::jar]
struct JarWithRetRef(MyInput);

#[salsa::jar]
struct JarWithDb(MyInput);

#[salsa::jar]
struct JarWithNoEq(MyInput);

#[salsa::jar]
struct JarWithJar(MyInput);

#[salsa::jar]
struct JarWithData(MyInput);

#[salsa::jar]
struct JarWithRecover(MyInput);

#[salsa::jar]
struct JarWithLru(MyInput);

#[salsa::jar]
struct JarWithConstructor(MyInput);

#[salsa::input(jar = Jar1)]
struct MyInput {
    field: u32,
}

fn main() {}
