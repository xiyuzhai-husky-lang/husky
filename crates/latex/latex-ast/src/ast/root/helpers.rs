use super::*;

pub enum LxRootAstChild {
    CommandArgument(LxRootCompleteCommandArgument),
}

impl LxRootAstData {
    pub fn children(&self) -> Vec<LxRootAstChild> {
        match self {
            LxRootAstData::CompleteCommand { ref arguments, .. } => arguments
                .iter()
                .copied()
                .map(LxRootAstChild::CommandArgument)
                .collect(),
            LxRootAstData::Environment(lx_root_environment_ast_data) => todo!(),
        }
    }
}
