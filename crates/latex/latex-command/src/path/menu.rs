use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct LxCommandPathMenu {
    // maths
    // - extended letters
    pub alpha: LxCommandPath,
    pub beta: LxCommandPath,
    pub gamma: LxCommandPath,
    // - constants
    pub pi: LxCommandPath,
    // - functions
    pub sin: LxCommandPath,
    pub cos: LxCommandPath,
    // - layouts
    pub sqrt: LxCommandPath,
    pub frac: LxCommandPath,
    // - environments
    pub text: LxCommandPath,
}

impl LxCommandPathMenu {
    fn new(db: &salsa::Db) -> Self {
        let p = |data: &str| LxCommandPath::new_prelude(Coword::from_ref(db, data), db);
        Self {
            alpha: p("alpha"),
            beta: p("beta"),
            gamma: p("gamma"),
            pi: p("pi"),
            sin: p("sin"),
            cos: p("cos"),
            sqrt: p("sqrt"),
            frac: p("frac"),
            text: p("text"),
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn command_path_menu(db: &salsa::Db) -> LxCommandPathMenu {
    LxCommandPathMenu::new(db)
}
