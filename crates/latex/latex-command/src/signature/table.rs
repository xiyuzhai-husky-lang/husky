use super::*;
use crate::path::{
    menu::{command_path_menu, LxCommandPathMenu},
    LxCommandName,
};
use latex_prelude::mode::LxMode;
use parameter::LxCommandParameter;
use rustc_hash::FxHashMap;

pub struct LxCommandSignatureTable {
    pub signatures: FxHashMap<LxCommandName, LxCommandSignature>,
}

impl std::ops::Deref for LxCommandSignatureTable {
    type Target = FxHashMap<LxCommandName, LxCommandSignature>;

    fn deref(&self) -> &Self::Target {
        &self.signatures
    }
}

impl LxCommandSignatureTable {
    pub fn new_default(db: &salsa::Db) -> Self {
        let menu = command_path_menu(db);
        Self {
            signatures: [
                (
                    menu.sqrt_name(),
                    LxCommandSignature {
                        path: menu.sqrt_path(),
                        parameters: vec![LxCommandParameter::new(LxMode::Math)],
                    },
                ),
                (
                    menu.sin_name(),
                    LxCommandSignature {
                        path: menu.sin_path(),
                        parameters: vec![],
                    },
                ),
                (
                    menu.cos_name(),
                    LxCommandSignature {
                        path: menu.cos_path(),
                        parameters: vec![],
                    },
                ),
            ]
            .into_iter()
            .collect(),
        }
    }
}
