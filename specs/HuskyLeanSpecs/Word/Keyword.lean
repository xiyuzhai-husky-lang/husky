import HuskyLeanSpecs.Word.Keyword.StmtKeyword
import HuskyLeanSpecs.Word.Keyword.TyKeyword
import HuskyLeanSpecs.Paradigm

inductive ConfigKeyword
  | Task


deriving instance BEq for ConfigKeyword

inductive LiasonKeyword

deriving instance BEq for LiasonKeyword

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

deriving instance BEq for Keyword