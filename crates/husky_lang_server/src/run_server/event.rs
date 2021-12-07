#[derive(Debug)]
pub(crate) enum Event {
    Lsp(lsp_server::Message),
}
