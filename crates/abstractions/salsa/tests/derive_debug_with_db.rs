#[salsa::derive_debug_with_db]
struct A(usize, usize);

#[salsa::derive_debug_with_db]
#[allow(dead_code)]
enum Enum {
    PropsStructVariant { a: i32 },
    TupleStructVariant(usize),
    Dog,
}

#[salsa::jar]
struct Jar();
