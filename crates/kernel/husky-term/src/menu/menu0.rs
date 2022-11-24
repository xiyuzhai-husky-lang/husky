use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu0 {
    sort: Term,
    universe1: Term,
    unit: Term,
    i32: Term,
    i64: Term,
    f32: Term,
    f64: Term,
    b32: Term,
    b64: Term,
    bool: Term,
    trai: Term,
    module: Term,
}

impl TermMenu0 {
    pub fn new(db: &dyn TermDb) -> Self {
        let sort = db.it_term(TermAtom::new_category(TermCategory::Sort).into());
        let universe1 = db.it_term(TermAtom::new_universe(1).into());
        let entity_path_menu = db.entity_path_menu();
        TermMenu0 {
            sort,
            universe1,
            unit: db.it_entity_path_term(entity_path_menu.unit()),
            i32: db.it_entity_path_term(entity_path_menu.i32()),
            i64: db.it_entity_path_term(entity_path_menu.i64()),
            f32: db.it_entity_path_term(entity_path_menu.f32()),
            f64: db.it_entity_path_term(entity_path_menu.f64()),
            b32: db.it_entity_path_term(entity_path_menu.b32()),
            b64: db.it_entity_path_term(entity_path_menu.b64()),
            bool: db.it_entity_path_term(entity_path_menu.bool()),
            trai: db.it_entity_path_term(entity_path_menu.trai()),
            module: db.it_entity_path_term(entity_path_menu.module()),
        }
    }

    pub fn sort(&self) -> Term {
        self.sort
    }

    pub fn universe1(&self) -> Term {
        self.universe1
    }

    pub fn i32(&self) -> Term {
        self.i32
    }

    pub fn unit(&self) -> Term {
        self.unit
    }

    pub fn i64(&self) -> Term {
        self.i64
    }

    pub fn f32(&self) -> Term {
        self.f32
    }

    pub fn f64(&self) -> Term {
        self.f64
    }

    pub fn b32(&self) -> Term {
        self.b32
    }

    pub fn b64(&self) -> Term {
        self.b64
    }

    pub fn bool(&self) -> Term {
        self.bool
    }

    pub fn trai(&self) -> Term {
        self.trai
    }

    pub fn module(&self) -> Term {
        self.module
    }
}
