inductive Paradigm
  | LazyFunctional
  | EagerFunctional
  | EagerProcedural

inductive ConfigKeyword
  | Task

inductive TyKeyword
  | Struct
  | Enum
  | Record

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
def as_str(kw: StmtKeyword): String :=
  match kw with
  | StmtKeyword.Let => "let"
  | StmtKeyword.Var => "var"
  | StmtKeyword.If => "if"
  | StmtKeyword.Elif => "elif"
  | StmtKeyword.Else => "else"
  | StmtKeyword.Match => "match"
  | StmtKeyword.Case => "case"
  | StmtKeyword.DeFault => "default"
  | StmtKeyword.For => "for"
  | StmtKeyword.ForExt => "forext"
  | StmtKeyword.While => "while"
  | StmtKeyword.Do => "do"
  | StmtKeyword.Break => "break"
  | StmtKeyword.Return => "return"
  | StmtKeyword.Assert => "assert"
  | StmtKeyword.Require => "require"
end StmtKeyword

inductive LiasonKeyword

inductive Keyword
    | Config: ConfigKeyword -> Keyword
    | Paradigm: Paradigm -> Keyword
    | Ty: TyKeyword -> Keyword
    | Stmt: StmtKeyword -> Keyword
    | Liason: LiasonKeyword -> Keyword
    | Main
    | Use
    | Mod
    | Visual


inductive WordOpr

inductive Decorator
  | Pub
  | Priviate
  | Async
  | Static

inductive WordPattern
  | Some
  | None

inductive Word
  | keyword: Keyword -> Word
  | Opr: WordOpr -> Word
  | Decorator: Decorator -> Word
  | Pattern: WordPattern -> Word
  | Identifier