use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct LxCommandPathMenu {
    // maths
    // - extended letters
    pub alpha_path: LxCommandPath,
    pub beta_path: LxCommandPath,
    pub gamma_path: LxCommandPath,
    // - constants
    pub pi_path: LxCommandPath,
    // - functions
    pub sin_path: LxCommandPath,
    pub cos_path: LxCommandPath,
    // - layouts
    pub sqrt_path: LxCommandPath,
    pub frac_path: LxCommandPath,
    // - environments
    pub text_path: LxCommandPath,
}

impl LxCommandPathMenu {
    fn new(db: &salsa::Db) -> Self {
        let p = |data: &str| LxCommandPath::new_prelude(Coword::from_ref(db, data), db);
        Self {
            alpha_path: p("alpha"),
            beta_path: p("beta"),
            gamma_path: p("gamma"),
            pi_path: p("pi"),
            sin_path: p("sin"),
            cos_path: p("cos"),
            sqrt_path: p("sqrt"),
            frac_path: p("frac"),
            text_path: p("text"),
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn command_path_menu(db: &salsa::Db) -> LxCommandPathMenu {
    LxCommandPathMenu::new(db)
}
