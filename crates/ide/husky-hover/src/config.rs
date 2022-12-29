use crate::*;

#[salsa::input(jar = HoverJar)]
pub struct HoverConfig {}

impl HoverConfig {
    pub fn hover_action_config(&self) -> &HoverActionsConfig {
        todo!()
    }
    pub fn client_commands_config(&self) -> &ClientCommandsConfig {
        todo!()
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
