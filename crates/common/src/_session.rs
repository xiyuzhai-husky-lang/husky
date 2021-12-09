// use super::*;
// use bimap::BiHashMap;

// pub struct Session {
//     symbols: BiHashMap<SymbolID, String>,
// }
// impl Session {
//     pub fn new() -> Session {
//         Session {
//             symbols: BiHashMap::<SymbolID, String>::new(),
//         }
//     }
//     pub fn get_symbol_id(&mut self, symbol_str: &str) -> SymbolID {
//         // td!()
//         match self.symbols.get_by_right(symbol_str) {
//             Some(id) => *id,
//             None => {
//                 let new_id = SymbolID::UserDefined(self.symbols.len());
//                 self.symbols.insert(new_id, symbol_str.to_string());
//                 new_id
//             }
//         }
//     }
//     pub fn get_symbol_str(&self, id: SymbolID) -> Option<&str> {
//         match id {
//             SymbolID::NotDefined => td!(),
//             SymbolID::UserDefined(_) => self.symbols.get_by_left(&id).map(|e| e.as_str()),
//             SymbolID::Keyword(keyword) => match keyword {
//                 Keyword::NoKeyword => td!(),
//                 Keyword::Use => td!(),
//                 Keyword::Include => td!(),
//                 Keyword::Pub => td!(),
//                 Keyword::Mod => td!(),
//                 Keyword::PubMod => td!(),
//                 Keyword::Def => td!(),
//                 Keyword::PubDef => td!(),
//                 Keyword::Func => td!(),
//                 Keyword::PubFunc => td!(),
//                 Keyword::Pattern => td!(),
//                 Keyword::PubPattern => td!(),
//                 Keyword::Struct => td!(),
//                 Keyword::PubStruct => td!(),
//                 Keyword::Rename => td!(),
//                 Keyword::PubRename => td!(),
//                 Keyword::Enum => td!(),
//                 Keyword::PubEnum => td!(),
//                 Keyword::Props => td!(),
//                 Keyword::PubProps => td!(),
//                 Keyword::Template => td!(),
//                 Keyword::PubTemplate => td!(),
//                 Keyword::Main => Some("main"),
//                 Keyword::Let => td!(),
//                 Keyword::Var => td!(),
//                 Keyword::If => td!(),
//                 Keyword::Elif => td!(),
//                 Keyword::Else => td!(),
//                 Keyword::Switch => td!(),
//                 Keyword::Case => td!(),
//                 Keyword::Default => td!(),
//                 Keyword::For => td!(),
//                 Keyword::Ext => td!(),
//                 Keyword::ForExt => td!(),
//                 Keyword::While => td!(),
//                 Keyword::Do => td!(),
//                 Keyword::DoWhile => td!(),
//                 Keyword::Break => td!(),
//                 Keyword::Return => td!(),
//                 Keyword::Comment => td!(),
//             },
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
// pub enum SymbolID {
//     NotDefined,
//     Keyword(Keyword),
//     UserDefined(usize),
// }
