use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct LxCommandPathMenu {
    // maths
    sqrt_name: LxCommandName,
    sqrt_path: LxCommandPath,
    sin_name: LxCommandName,
    sin_path: LxCommandPath,
    cos_name: LxCommandName,
    cos_path: LxCommandPath,
}

impl LxCommandPathMenu {
    fn new(db: &salsa::Db) -> Self {
        let n = |data: &str| LxCommandName::new2(data, db).unwrap();
        let p = |data: &str| LxCommandPath::new_prelude(Coword::from_ref(db, data), db);
        Self {
            sqrt_name: n("sqrt"),
            sqrt_path: p("sqrt"),
            sin_name: n("sin"),
            sin_path: p("sin"),
            cos_name: n("cos"),
            cos_path: p("cos"),
        }
    }
}

impl LxCommandPathMenu {
    pub fn sqrt_name(&self) -> LxCommandName {
        self.sqrt_name
    }

    pub fn sqrt_path(&self) -> LxCommandPath {
        self.sqrt_path
    }

    pub fn sin_name(&self) -> LxCommandName {
        self.sin_name
    }

    pub fn sin_path(&self) -> LxCommandPath {
        self.sin_path
    }

    pub fn cos_name(&self) -> LxCommandName {
        self.cos_name
    }

    pub fn cos_path(&self) -> LxCommandPath {
        self.cos_path
    }
}

#[salsa::tracked(return_ref)]
pub fn command_path_menu(db: &salsa::Db) -> LxCommandPathMenu {
    LxCommandPathMenu::new(db)
}
