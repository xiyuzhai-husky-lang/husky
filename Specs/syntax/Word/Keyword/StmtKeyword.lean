
inductive StmtKeyword
  | Let
  | Var
  | If
  | Elif
  | Else
  | Match
  | Case
  | DeFault
  | For
  | ForExt
  | While
  | Do
  | Break
  | Return
  | Assert
  | Require
  deriving DecidableEq

namespace StmtKeyword
def kindName : StmtKeyword -> String
  | Let => "Let"
  | Var => "Var"
  | If => "If"
  | Elif => "Elif"
  | Else => "Else"
  | Match => "Match"
  | Case => "Case"
  | DeFault => "Default"
  | For => "For"
  | ForExt => "ForExt"
  | While => "While"
  | Do => "Do"
  | Break => "Break"
  | Return => "Return"
  | Assert => "Assert"
  | Require => "Require"

def huskyCode : StmtKeyword -> String
  | Let => "let"
  | Var => "var"
  | If => "if"
  | Elif => "elif"
  | Else => "else"
  | Match => "match"
  | Case => "case"
  | DeFault => "default"
  | For => "for"
  | ForExt => "forext"
  | While => "while"
  | Do => "do"
  | Break => "break"
  | Return => "return"
  | Assert => "assert"
  | Require => "require"

def StmtKeywordEnumeration := [
  Let,
  Var,
  If,
  Elif,
  Else,
  Match,
  Case,
  DeFault,
  For,
  ForExt,
  While,
  Do,
  Break,
  Return,
  Assert,
  Require
]


instance : ToString StmtKeyword where
  toString : StmtKeyword -> String
  | kw => s!"StmtKeyword::{kw.kindName}"
end StmtKeyword
