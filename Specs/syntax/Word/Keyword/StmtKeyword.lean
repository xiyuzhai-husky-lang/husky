
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
