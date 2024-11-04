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

impl<const N: usize> From<[(LxCommandPath, &[LxMode]); N]> for LxCommandSignatureTable {
    fn from(value: [(LxCommandPath, &[LxMode]); N]) -> Self {
        Self {
            signatures: value
                .into_iter()
                .map(|(path, parameter_modes)| {
                    (
                        path.name(),
                        LxCommandSignature {
                            path,
                            parameters: parameter_modes
                                .into_iter()
                                .copied()
                                .map(LxCommandParameter::new)
                                .collect(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl<const N: usize> From<[LxCommandSignature; N]> for LxCommandSignatureTable {
    fn from(value: [LxCommandSignature; N]) -> Self {
        Self {
            signatures: value
                .into_iter()
                .map(|signature| (signature.path().name(), signature))
                .collect(),
        }
    }
}

impl std::ops::Deref for LxCommandSignatureTable {
    type Target = FxHashMap<LxCommandName, LxCommandSignature>;

    fn deref(&self) -> &Self::Target {
        &self.signatures
    }
}

impl LxCommandSignatureTable {
    pub fn new_default(db: &salsa::Db) -> Self {
        use LxMode::*;

        let LxCommandPathMenu {
            sqrt_path,
            sin_path,
            cos_path,
            frac_path,
            text_path,
            ..
        } = command_path_menu(db);
        [
            (sqrt_path, &[Math]),
            (sin_path, &[]),
            (cos_path, &[]),
            (frac_path, &[Math, Math]),
            (text_path, &[Rose]),
        ]
        .into()
    }
}
