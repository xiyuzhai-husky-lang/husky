use crate::task::Task;

#[derive(Debug)]
pub(crate) enum Event {
    Lsp(lsp_server::Message),
    Task(Task),
    Vfs(vfs::loader::Message),
    Flycheck(flycheck::Message),
}
