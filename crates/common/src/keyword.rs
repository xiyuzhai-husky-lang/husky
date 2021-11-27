#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
  NoKeyword,
  Use,         // use
  Include,     // incl .
  Pub,         // pub
  Mod,         // mod
  PubMod,      // pub mod
  Def,         // def
  PubDef,      // pub def
  Func,        // func
  PubFunc,     // pub func
  Pattern,     // pattern
  PubPattern,  // pub pattern
  Struct,      // struct
  PubStruct,   // pub struct
  Rename,      // struct
  PubRename,   // pub struct
  Enum,        // enum
  PubEnum,     // enum
  Props,       // props
  PubProps,    // pub props
  Template,    // tmpl
  PubTemplate, // pub tmpl
  Main,        // main
  Let,         // let
  Var,         // var
  If,          // if
  Elif,        // elif
  Else,        // else
  Switch,      // switch
  Case,        // case
  Default,     // default
  For,         // for
  Ext,         // ext
  ForExt,      // for ext
  While,       // while
  Do,          // do
  DoWhile,     // do while
  Break,       // break
  Return,      // return
  Comment,     // //
}
