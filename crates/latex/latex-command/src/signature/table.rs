use super::*;
use crate::path::{
    menu::{command_path_menu, LxCommandPathMenu},
    LxCommandName,
};
use latex_prelude::mode::LxMode;
use parameter::{LxCommandParameter, LxCommandParameterMode};
use rustc_hash::FxHashMap;

#[salsa::derive_debug_with_db]
#[derive(Debug)]
pub struct LxCommandSignatureTable {
    pub signatures: FxHashMap<LxCommandName, LxCommandSignature>,
}

impl LxCommandSignatureTable {
    // TODO: return a closest match if the command is not found
    pub fn signature(&self, index: LxCommandName) -> Option<&LxCommandSignature> {
        self.signatures.get(&index)
    }
}

impl LxCommandSignatureTable {
    fn new(
        begin: LxCommandPath,
        end: LxCommandPath,
        letter_style_commands: &[(LxCommandPath, LxMathLetterStyle)],
        complete_commands: &[(LxCommandPath, &[LxMode], &[LxCommandParameterMode])],
    ) -> Self {
        Self {
            signatures: [
                (begin.name(), LxCommandSignature::Begin),
                (end.name(), LxCommandSignature::End),
            ]
            .into_iter()
            .chain(
                letter_style_commands
                    .iter()
                    .copied()
                    .map(|(path, style)| (path.name(), LxCommandSignature::MathLetterStyle(style))),
            )
            .chain(complete_commands.into_iter().copied().map(
                |(path, allowed_modes, parameter_modes)| {
                    (
                        path.name(),
                        LxCommandSignature::Complete(LxCompleteCommandSignature {
                            path,
                            allowed_modes: allowed_modes.into(),
                            options: (),
                            parameters: parameter_modes
                                .into_iter()
                                .copied()
                                .map(LxCommandParameter::new)
                                .collect(),
                        }),
                    )
                },
            ))
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
        let LxCommandPathMenu {
            // - root
            begin,
            end,
            usepackage,
            documentclass,
            newtheorem,
            // - divisions
            part,
            chapter,
            section,
            subsection,
            subsubsection,
            // - maths
            // ## letter style
            mathbb,
            mathbf,
            mathcal,
            mathit,
            mathrm,
            mathsf,
            mathscr,
            // - operators
            // -- relations
            eq,
            ne,
            le,
            ge,
            r#in,
            subset,
            supset,
            subseteq,
            supseteq,
            subseteqq,
            supseteqq,
            subsetneq,
            supsetneq,
            // -- arithmetic
            int,
            sum,
            prod,
            times,
            otimes,
            // -- extended letters
            alpha,
            beta,
            gamma,
            pi,
            // -- functions
            sin,
            cos,
            // -- layouts
            sqrt,
            frac,
            text,
        } = *command_path_menu(db);
        Self::new(
            begin,
            end,
            &[
                (mathbb, LxMathLetterStyle::MATHBB),
                (mathbf, LxMathLetterStyle::MATHFRAK),
                (mathcal, LxMathLetterStyle::MATHCAL),
                (mathit, LxMathLetterStyle::MATHIT),
                (mathrm, LxMathLetterStyle::MATHRM),
                (mathsf, LxMathLetterStyle::MATHSF),
                (mathscr, LxMathLetterStyle::MATHSCR),
            ],
            &[
                // - root
                (usepackage, &[LxMode::Root], &[LxCommandParameterMode::Name]),
                (
                    documentclass,
                    &[LxMode::Root],
                    &[LxCommandParameterMode::Name],
                ),
                (
                    newtheorem,
                    &[LxMode::Root],
                    &[LxCommandParameterMode::Name, LxCommandParameterMode::Name],
                ),
                // - divisions
                (part, &[LxMode::Root], &[LxCommandParameterMode::Rose]),
                (chapter, &[LxMode::Root], &[LxCommandParameterMode::Rose]),
                (section, &[LxMode::Root], &[LxCommandParameterMode::Rose]),
                (subsection, &[LxMode::Root], &[LxCommandParameterMode::Rose]),
                (
                    subsubsection,
                    &[LxMode::Root],
                    &[LxCommandParameterMode::Rose],
                ),
                // - operators
                // -- relations
                (eq, &[LxMode::Math], &[]),
                (ne, &[LxMode::Math], &[]),
                (le, &[LxMode::Math], &[]),
                (ge, &[LxMode::Math], &[]),
                (r#in, &[LxMode::Math], &[]),
                (subset, &[LxMode::Math], &[]),
                (supset, &[LxMode::Math], &[]),
                (subseteq, &[LxMode::Math], &[]),
                (supseteq, &[LxMode::Math], &[]),
                (subseteqq, &[LxMode::Math], &[]),
                (supseteqq, &[LxMode::Math], &[]),
                (subsetneq, &[LxMode::Math], &[]),
                (supsetneq, &[LxMode::Math], &[]),
                // -- arithmetic
                (int, &[LxMode::Math], &[]),
                (sum, &[LxMode::Math], &[]),
                (prod, &[LxMode::Math], &[]),
                (times, &[LxMode::Math], &[]),
                (otimes, &[LxMode::Math], &[]),
                // -- extended letters
                (alpha, &[LxMode::Math], &[]),
                (beta, &[LxMode::Math], &[]),
                (gamma, &[LxMode::Math], &[]),
                (pi, &[LxMode::Math], &[]),
                // -- functions
                (sqrt, &[LxMode::Math], &[LxCommandParameterMode::Math]),
                (sin, &[LxMode::Math], &[]),
                (cos, &[LxMode::Math], &[]),
                // -- layouts
                (
                    frac,
                    &[LxMode::Math],
                    &[LxCommandParameterMode::Math, LxCommandParameterMode::Math],
                ),
                (text, &[LxMode::Math], &[LxCommandParameterMode::Rose]),
            ],
        )
    }
}
