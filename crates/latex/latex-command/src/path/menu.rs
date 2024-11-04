use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct LxCommandPathMenu {
    // maths
    pub sqrt_name: LxCommandName,
    pub sqrt_path: LxCommandPath,
    pub sin_name: LxCommandName,
    pub sin_path: LxCommandPath,
    pub cos_name: LxCommandName,
    pub cos_path: LxCommandPath,
    pub frac_name: LxCommandName,
    pub frac_path: LxCommandPath,
    pub text_name: LxCommandName,
    pub text_path: LxCommandPath,
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
            frac_name: n("frac"),
            frac_path: p("frac"),
            text_name: n("text"),
            text_path: p("text"),
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

    pub fn frac_name(&self) -> LxCommandName {
        self.frac_name
    }

    pub fn frac_path(&self) -> LxCommandPath {
        self.frac_path
    }

    pub fn text_name(&self) -> LxCommandName {
        self.text_name
    }

    pub fn text_path(&self) -> LxCommandPath {
        self.text_path
    }
}

#[salsa::tracked(return_ref)]
pub fn command_path_menu(db: &salsa::Db) -> LxCommandPathMenu {
    LxCommandPathMenu::new(db)
}
