pub mod environment;
pub mod helpers;
#[cfg(test)]
pub mod tests;

use self::environment::LxRootEnvironmentAstData;
use super::*;
use husky_coword::Coword;
use latex_command::{
    path::{LxCommandName, LxCommandPath},
    signature::{
        parameter::LxCommandParameterMode, LxCommandSignature, LxCompleteCommandSignature,
    },
};
use latex_token::{
    idx::{LxNameTokenIdx, LxRootTokenIdx},
    token::{
        name::LxNameTokenData,
        root::{LxRootDelimiter, LxRootTokenData},
    },
};
use smallvec::{smallvec, SmallVec};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum LxRootAstData {
    CompleteCommand {
        command_token_idx: LxRootTokenIdx,
        command_path: LxCommandPath,
        arguments: SmallVec<[LxRootCompleteCommandArgument; 2]>,
    },
    Environment(LxRootEnvironmentAstData),
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxRootCompleteCommandArgument {
    lcurl_token_idx: LxRootTokenIdx,
    data: LxRootCommandArgumentData,
    rcurl_token_idx: LxRootTokenIdx,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LxRootCommandArgumentData {
    Name(LxNameTokenIdx, Coword),
}

pub type LxRootAstArena = Arena<LxRootAstData>;
pub type LxRootAstArenaRef<'a> = ArenaRef<'a, LxRootAstData>;
pub type LxRootAstIdx = ArenaIdx<LxRootAstData>;
pub type LxRootAstIdxRange = ArenaIdxRange<LxRootAstData>;
pub type LxRootAstArenaMap<T> = ArenaMap<LxRootAstData, T>;

impl LxRootCompleteCommandArgument {
    pub fn lcurl_token_idx(self) -> LxRootTokenIdx {
        self.lcurl_token_idx
    }

    pub fn data(self) -> LxRootCommandArgumentData {
        self.data
    }

    pub fn rcurl_token_idx(self) -> LxRootTokenIdx {
        self.rcurl_token_idx
    }
}

impl<'a> LxAstParser<'a> {
    pub(super) fn parse_root_asts(&mut self) -> LxRootAstIdxRange {
        let mut asts = vec![];
        while let Some(ast) = self.parse_root_ast() {
            asts.push(ast)
        }
        self.alloc_root_asts(asts)
    }

    fn parse_root_ast(&mut self) -> Option<LxRootAstData> {
        match self.peek_root_token_data()? {
            LxRootTokenData::RightDelimiter(_) => return None,
            // TODO: this is a hack
            LxRootTokenData::Command(command_name)
                if command_name == self.command_path_menu().end.name() =>
            {
                return None
            }
            _ => (),
        };
        let (token_idx, token) = self.next_root_token()?;
        match token {
            LxRootTokenData::Command(command_name) => {
                self.parse_root_command(token_idx, command_name)
            }
            LxRootTokenData::LeftDelimiter(root_delimiter) => todo!(),
            LxRootTokenData::RightDelimiter(root_delimiter) => todo!(),
        }
    }

    fn parse_root_command(
        &mut self,
        token_idx: LxRootTokenIdx,
        command_name: LxCommandName,
    ) -> Option<LxRootAstData> {
        let Some(command_signature) = self.command_signature_table().signature(command_name) else {
            use salsa::DisplayWithDb;
            todo!(
                "handle command `{}` not found in command signature table",
                command_name.display(self.db())
            )
        };
        match *command_signature {
            LxCommandSignature::Complete(ref command_signature) => {
                self.parse_root_complete_command(token_idx, command_signature)
            }
            LxCommandSignature::MathLetterStyle(style) => {
                todo!()
            }
            LxCommandSignature::Begin => todo!(),
            LxCommandSignature::End => todo!(),
        }
    }

    fn parse_root_complete_command(
        &mut self,
        command_token_idx: LxRootTokenIdx,
        command_signature: &LxCompleteCommandSignature,
    ) -> Option<LxRootAstData> {
        let command_path = command_signature.path();
        let mut arguments: SmallVec<[LxRootCompleteCommandArgument; 2]> = smallvec![];
        for parameter in command_signature.parameters() {
            arguments.push(self.parse_root_complete_command_argument(parameter.mode()));
        }
        Some(LxRootAstData::CompleteCommand {
            command_token_idx,
            command_path,
            arguments,
        })
    }

    fn parse_root_complete_command_argument(
        &mut self,
        mode: LxCommandParameterMode,
    ) -> LxRootCompleteCommandArgument {
        let Some((lcurl_token_idx, LxRootTokenData::LeftDelimiter(LxRootDelimiter::Curl))) =
            self.next_root_token()
        else {
            todo!("report errors properly")
        };

        let data = match mode {
            LxCommandParameterMode::Math => todo!(),
            LxCommandParameterMode::Rose => todo!(),
            LxCommandParameterMode::Name => {
                let Some((name_token_idx, LxNameTokenData::Name(name))) = self.next_name_token()
                else {
                    todo!("report errors properly")
                };
                LxRootCommandArgumentData::Name(name_token_idx, name)
            }
            LxCommandParameterMode::SingleLetter => todo!(),
        };
        let Some((rcurl_token_idx, LxRootTokenData::RightDelimiter(LxRootDelimiter::Curl))) =
            self.next_root_token()
        else {
            todo!("report errors properly")
        };
        LxRootCompleteCommandArgument {
            lcurl_token_idx,
            data,
            rcurl_token_idx,
        }
    }
}
