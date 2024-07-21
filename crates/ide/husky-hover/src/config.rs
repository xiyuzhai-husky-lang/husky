use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct HoverConfig {
    pub debug: bool,
    pub description: bool,
    // lex
    pub token_idx: bool,
    pub token_line_group_idx: bool,
    pub token: bool,
    pub token_info: bool,
    // syntax
    // semantics
    pub coersion: bool,
    pub ty: bool,
}

impl HoverConfig {
    pub fn hover_action_config(&self) -> &HoverActionsConfig {
        todo!()
    }
    pub fn client_commands_config(&self) -> &ClientCommandsConfig {
        todo!()
    }
}

#[salsa::tracked(return_ref)]
pub(crate) fn hover_config(db: &::salsa::Db, module_path: ModulePath) -> HoverConfig {
    HoverConfig {
        debug: true,
        description: true,
        token_idx: false,
        token_line_group_idx: false,
        token: false,
        token_info: false,
        coersion: true,
        ty: true,
    }
}

pub struct HoverActionsConfig {
    enable_implementations: bool,
    enable_references: bool,
    enable_run: bool,
    enable_debug: bool,
    enable_goto_type_def: bool,
}

impl HoverActionsConfig {
    pub fn enable_implementations(&self) -> bool {
        self.enable_implementations
    }

    pub fn enable_references(&self) -> bool {
        self.enable_references
    }

    pub fn enable_run(&self) -> bool {
        self.enable_run
    }

    pub fn enable_goto_type_def(&self) -> bool {
        self.enable_goto_type_def
    }
}

pub struct ClientCommandsConfig {}

impl ClientCommandsConfig {
    pub fn show_reference(&self) -> bool {
        todo!()
    }
}
