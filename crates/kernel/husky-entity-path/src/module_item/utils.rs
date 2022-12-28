use crate::*;

pub(super) fn show_connection(connection: ModuleItemConnection) -> &'static str {
    match connection {
        ModuleItemConnection::Connected => "::",
        ModuleItemConnection::Disconnected => ":/:",
    }
}
