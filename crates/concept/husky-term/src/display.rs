use std::ops::Deref;

use crate::*;

// mom
impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Literal(l) => l.fmt(f),
            Term::Entity(e) => e.fmt(f),
            Term::Curry(c) => c.fmt(f),
            Term::Variable(v) => v.fmt(f),
            Term::Abstraction(a) => a.fmt(f),
            Term::Application(a) => a.fmt(f),
            Term::Universe(a) => a.fmt(f),
        }
    }
}

impl std::fmt::Display for TermLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data() {
            TermLiteralData::Void => "()".fmt(f),
            TermLiteralData::I32(v) => v.fmt(f),
            TermLiteralData::I64(v) => v.fmt(f),
            TermLiteralData::Float(v) => v.fmt(f),
            TermLiteralData::F32(v) => v.fmt(f),
            TermLiteralData::F64(v) => v.fmt(f),
            TermLiteralData::Bits(v) => v.fmt(f),
            TermLiteralData::B32(v) => v.fmt(f),
            TermLiteralData::B64(v) => v.fmt(f),
            TermLiteralData::Bool(v) => v.fmt(f),
        }
    }
}

impl std::fmt::Display for TermEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.path().fmt(f)
    }
}

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}
impl std::fmt::Display for TermCurry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::fmt::Display for TermVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::fmt::Display for TermAbstraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::fmt::Display for TermApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::fmt::Display for TermUniverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[test]
fn display_term() {
    let db = TermTestsDb::new();
    let menu = db.term_menu();
    // TermLiteralData::I32(v) => v.fmt(f),
    assert_eq!(menu.i32().to_string(), "i32");
    // TermLiteralData::I64(v) => v.fmt(f),
    assert_eq!(menu.i64().to_string(), "i64");
    // TermLiteralData::F32(v) => v.fmt(f),
    assert_eq!(menu.f32().to_string(), "f32");
    // TermLiteralData::F64(v) => v.fmt(f),
    assert_eq!(menu.f64().to_string(), "f64");
    // TermLiteralData::B32(v) => v.fmt(f),
    assert_eq!(menu.b64().to_string(), "b64");
    // TermLiteralData::B64(v) => v.fmt(f),
    assert_eq!(menu.b64().to_string(), "b64");
    // TermLiteralData::Bool(v) => v.fmt(f),
    assert_eq!(menu.bool().to_string(), "bool");
    // mom
}
