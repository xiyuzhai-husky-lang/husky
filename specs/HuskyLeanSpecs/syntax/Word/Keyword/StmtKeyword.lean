
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

deriving instance BEq for StmtKeyword
deriving instance DecidableEq for StmtKeyword

def as_str(kw: StmtKeyword): String :=
  match kw with
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
