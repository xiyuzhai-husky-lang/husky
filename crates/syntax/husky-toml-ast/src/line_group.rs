use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TomlLineGroup {
    SectionTitle(Vec<Word>),
    KeyValue(Word, TomlExprIdx),
    Comment,
    Err,
}

// use crate::*;
// use husky_toml_token::{TomlToken, TomlTokenVariant};

// fn produce_line_group_starts(tokens: &[TomlToken]) -> Vec<usize> {
//     let line_starts = produce_line_starts(tokens);
//     let mut i = 0;
//     let mut line_group_starts = vec![];
//     while i < line_starts.len() {
//         let line_start0 = line_starts[i];
//         let line_indent0 = tokens[line_start0].range().start.col.0;
//         line_group_starts.push(i);
//         i = {
//             let mut j = i + 1;
//             while j < line_starts.len() {
//                 let line_start1 = line_starts[j];
//                 let line_indent1 = tokens[line_start1].range().start.col.0;
//                 todo!()
//                 // if line_indent1 > line_indent0 {
//                 //     continue;
//                 // } else if line_indent1 = line_indent0 {
//                 // } else
//             }
//             j
//         }
//     }
//     line_group_starts
// }

// fn produce_line_starts(tokens: &[TomlToken]) -> Vec<usize> {
//     (0..tokens.len())
//         .filter_map(|line_start| {
//             if line_start == 0 {
//                 Some(0)
//             } else if tokens[line_start - 1].range().end.line
//                 < tokens[line_start].range().start.line
//             {
//                 Some(line_start)
//             } else {
//                 None
//             }
//         })
//         .collect()
// }
