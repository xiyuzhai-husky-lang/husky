use ide_db::{base_db::Upcast, defs::Definition, helpers::pick_best_token, RootDatabase};
use syntax::{ast, match_ast, AstNode, SyntaxKind::*, SyntaxToken, T};

use crate::{FilePosition, NavigationTarget, RangeInfo, TryToNav};

// Feature: Go to Type Definition
//
// Navigates to the type of an identifier.
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Go to Type Definition*
// |===
//
// image::https://user-images.githubusercontent.com/48062697/113020657-b560f500-917a-11eb-9007-0f809733a338.gif[]
pub(crate) fn goto_type_definition(
    db: &RootDatabase,
    position: FilePosition,
) -> Option<RangeInfo<Vec<NavigationTarget>>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use ide_db::base_db::FileRange;
    use itertools::Itertools;

    use crate::fixture;

    fn check(ra_fixture: &str) {
        let (analysis, position, expected) = fixture::annotations(ra_fixture);
        let navs = analysis
            .goto_type_definition(position)
            .unwrap()
            .unwrap()
            .info;
        assert_ne!(navs.len(), 0);

        let cmp = |&FileRange { file_id, range }: &_| (file_id, range.start());
        let navs = navs
            .into_iter()
            .map(|nav| FileRange {
                file_id: nav.file_id,
                range: nav.focus_or_full_range(),
            })
            .sorted_by_key(cmp)
            .collect::<Vec<_>>();
        let expected = expected
            .into_iter()
            .map(|(FileRange { file_id, range }, _)| FileRange { file_id, range })
            .sorted_by_key(cmp)
            .collect::<Vec<_>>();
        assert_eq!(expected, navs);
    }

    #[test]
    fn goto_type_definition_works_simple() {
        check(
            r#"
struct Foo;
     //^^^
fn foo() {
    let f: Foo; f$0
}
"#,
        );
    }

    #[test]
    fn goto_type_definition_record_expr_field() {
        check(
            r#"
struct Bar;
    // ^^^
struct Foo { foo: Bar }
fn foo() {
    Foo { foo$0 }
}
"#,
        );
        check(
            r#"
struct Bar;
    // ^^^
struct Foo { foo: Bar }
fn foo() {
    Foo { foo$0: Bar }
}
"#,
        );
    }

    #[test]
    fn goto_type_definition_record_pat_field() {
        check(
            r#"
struct Bar;
    // ^^^
struct Foo { foo: Bar }
fn foo() {
    let Foo { foo$0 };
}
"#,
        );
        check(
            r#"
struct Bar;
    // ^^^
struct Foo { foo: Bar }
fn foo() {
    let Foo { foo$0: bar };
}
"#,
        );
    }

    #[test]
    fn goto_type_definition_works_simple_ref() {
        check(
            r#"
struct Foo;
     //^^^
fn foo() {
    let f: &Foo; f$0
}
"#,
        );
    }

    #[test]
    fn goto_type_definition_works_through_macro() {
        check(
            r#"
macro_rules! id { ($($tt:tt)*) => { $($tt)* } }
struct Foo {}
     //^^^
id! {
    fn bar() { let f$0 = Foo {}; }
}
"#,
        );
    }

    #[test]
    fn goto_type_definition_for_param() {
        check(
            r#"
struct Foo;
     //^^^
fn foo($0f: Foo) {}
"#,
        );
    }

    #[test]
    fn goto_type_definition_for_tuple_field() {
        check(
            r#"
struct Foo;
     //^^^
struct Bar(Foo);
fn foo() {
    let bar = Bar(Foo);
    bar.$00;
}
"#,
        );
    }

    #[test]
    fn goto_def_for_self_param() {
        check(
            r#"
struct Foo;
     //^^^
impl Foo {
    fn f(&self$0) {}
}
"#,
        )
    }

    #[test]
    fn goto_def_for_type_fallback() {
        check(
            r#"
struct Foo;
     //^^^
impl Foo$0 {}
"#,
        )
    }

    #[test]
    fn goto_def_for_struct_field() {
        check(
            r#"
struct Bar;
     //^^^

struct Foo {
    bar$0: Bar,
}
"#,
        );
    }

    #[test]
    fn goto_def_for_enum_struct_field() {
        check(
            r#"
struct Bar;
     //^^^

enum Foo {
    Bar {
        bar$0: Bar
    },
}
"#,
        );
    }

    #[test]
    fn goto_def_considers_generics() {
        check(
            r#"
struct Foo;
     //^^^
struct Bar<T, U>(T, U);
     //^^^
struct Baz<T>(T);
     //^^^

fn foo(x$0: Bar<Baz<Foo>, Baz<usize>) {}
"#,
        );
    }
}
