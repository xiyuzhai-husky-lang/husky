import HuskyLeanSpecs.syntax.Word.Keyword.StmtKeyword
import HuskyLeanSpecs.syntax.Word.Keyword.TyKeyword
import HuskyLeanSpecs.syntax.Paradigm

inductive ConfigKeyword
  | Task
  deriving BEq

inductive LiasonKeyword
  | Haha
  deriving BEq
 

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
  deriving BEq
