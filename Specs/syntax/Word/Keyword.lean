import Specs.syntax.Word.Keyword.StmtKeyword
import Specs.syntax.Word.Keyword.TyKeyword
import Specs.syntax.Paradigm

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

namespace Keyword
end Keyword