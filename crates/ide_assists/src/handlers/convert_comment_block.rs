use itertools::Itertools;
use syntax::{
    ast::{self, edit::IndentLevel, Comment},
    Direction, SyntaxElement, TextRange,
};

use crate::{AssistContext, AssistId, AssistKind, Assists};

pub(crate) fn convert_comment_block(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

fn block_to_line(acc: &mut Assists, comment: ast::Comment) -> Option<()> {
    todo!()
}

fn line_to_block(acc: &mut Assists, comment: ast::Comment) -> Option<()> {
    todo!()
}

/// The line -> block assist can  be invoked from anywhere within a sequence of line comments.
/// relevant_line_comments crawls backwards and forwards finding the complete sequence of comments that will
/// be joined.
fn relevant_line_comments(comment: &ast::Comment) -> Vec<Comment> {
    todo!()
}

fn line_comment_text(indentation: IndentLevel, comm: ast::Comment) -> String {
    todo!()
}
