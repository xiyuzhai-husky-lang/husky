use base_db::{fixture::ChangeFixture, FilePosition};
use expect_test::{expect, Expect};
use hir::Semantics;
use syntax::ast::{self, AstNode};

use crate::RootDatabase;

/// Creates analysis from a multi-file fixture, returns positions marked with $0.
pub(crate) fn position(ra_fixture: &str) -> (RootDatabase, FilePosition) {
    let change_fixture = ChangeFixture::parse(ra_fixture);
    let mut database = RootDatabase::default();
    database.apply_change(change_fixture.change);
    let (file_id, range_or_offset) = change_fixture
        .file_position
        .expect("expected a marker ($0)");
    let offset = range_or_offset.expect_offset();
    (database, FilePosition { file_id, offset })
}

fn check_trait(ra_fixture: &str, expect: Expect) {
    todo!()
}

fn check_missing_assoc(ra_fixture: &str, expect: Expect) {
    todo!()
}

#[test]
fn resolve_trait() {
    check_trait(
        r#"
pub trait Foo {
    fn bar();
}
impl Foo for u8 {
    $0
}
            "#,
        expect![["Foo"]],
    );
    check_trait(
        r#"
pub trait Foo {
    fn bar();
}
impl Foo for u8 {
    fn bar() {
        fn baz() {
            $0
        }
        baz();
    }
}
            "#,
        expect![["Foo"]],
    );
    check_trait(
        r#"
pub trait Foo {
    fn bar();
}
pub struct Bar;
impl Bar {
    $0
}
            "#,
        expect![[""]],
    );
}

#[test]
fn missing_assoc_items() {
    check_missing_assoc(
        r#"
pub trait Foo {
    const FOO: u8;
    fn bar();
}
impl Foo for u8 {
    $0
}"#,
        expect![[r#"
                FOO
                bar"#]],
    );

    check_missing_assoc(
        r#"
pub trait Foo {
    const FOO: u8;
    fn bar();
}
impl Foo for u8 {
    const FOO: u8 = 10;
    $0
}"#,
        expect![[r#"
                bar"#]],
    );

    check_missing_assoc(
        r#"
pub trait Foo {
    const FOO: u8;
    fn bar();
}
impl Foo for u8 {
    const FOO: u8 = 10;
    fn bar() {$0}
}"#,
        expect![[r#""#]],
    );

    check_missing_assoc(
        r#"
pub struct Foo;
impl Foo {
    fn bar() {$0}
}"#,
        expect![[r#""#]],
    );
}
