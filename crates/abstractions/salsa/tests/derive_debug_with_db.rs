#[cfg(test)]
use expect_test::*;

#[cfg(test)]
fn t<T>(t: T, expected: Expect, db: &::salsa::Db)
where
    T: ::salsa::DebugWithDb,
{
    expected.assert_debug_eq(&t.debug(db))
}

#[salsa::derive_debug_with_db]
struct A(usize, usize);

#[test]
fn struct_derive_debug_with_db_works() {
    use expect_test::*;

    let db = &salsa::Db::new(|_, _| ());
    t(
        A(0, 1),
        expect![[r#"
        A(
            0,
            1,
        )
    "#]],
        db,
    );
}

#[salsa::derive_debug_with_db]
enum Enum {
    PropsStructVariant { a: i32 },
    TupleStructVariant(usize),
    Dog,
}

#[test]
fn enum_derive_debug_with_db_works() {
    use expect_test::*;

    let db = &salsa::Db::new(|_, _| ());
    t(
        Enum::PropsStructVariant { a: 1 },
        expect![[r#"
        Enum::PropsStructVariant {
            a: 1,
        }
    "#]],
        db,
    );
    t(
        Enum::TupleStructVariant(0),
        expect![[r#"
        Enum::TupleStructVariant(
            0,
        )
    "#]],
        db,
    );
    t(
        Enum::Dog,
        expect![[r#"
        Enum::Dog
    "#]],
        db,
    );
}
