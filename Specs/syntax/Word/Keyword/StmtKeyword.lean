
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
  deriving BEq

namespace StmtKeyword
def toRustVersion : StmtKeyword -> String
  | Let => "StmtKeyword::Let"
  | Var => "StmtKeyword::Var"
  | If => "StmtKeyword::If"
  | Elif => "StmtKeyword::Elif"
  | Else => "StmtKeyword::Else"
  | Match => "StmtKeyword::Match"
  | Case => "StmtKeyword::Case"
  | DeFault => "StmtKeyword::Default"
  | For => "StmtKeyword::For"
  | ForExt => "StmtKeyword::ForExt"
  | While => "StmtKeyword::While"
  | Do => "StmtKeyword::Do"
  | Break => "StmtKeyword::Break"
  | Return => "StmtKeyword::Return"
  | Assert => "StmtKeyword::Assert"
  | Require => "StmtKeyword::Require"
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
end StmtKeyword
