mod builder;
mod cursor_range;
mod output;
mod state;
mod text_buffer;

pub use {
    builder::CodeEdit, cursor_range::*, output::TextEditOutput, state::CodeEditState,
    text_buffer::TextBuffer,
};

use egui::*;

#[test]
fn haha() {}
