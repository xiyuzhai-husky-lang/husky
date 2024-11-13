use super::*;

pub enum LxLispAstChild {
    LispAst(LxLispAstIdx),
}

impl LxLispAstData {
    pub fn children(&self) -> Vec<LxLispAstChild> {
        match *self {
            LxLispAstData::Ident(_, _) => vec![],
            LxLispAstData::CompleteCommand {
                command_token_idx,
                command_path,
                ref arguments,
            } => todo!(),
            LxLispAstData::Parenthesized {
                lpar_token_idx,
                asts,
                rpar_token_idx,
            } => asts.into_iter().map(LxLispAstChild::LispAst).collect(),
        }
    }
}
