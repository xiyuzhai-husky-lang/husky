import Specs.syntax.Word.Keyword.StmtKeyword
import Specs.syntax.Word.Keyword.TyKeyword
import Specs.syntax.Paradigm

inductive ConfigKeyword
  | Task
  deriving BEq
namespace ConfigKeyword
instance : ToString ConfigKeyword where
  toString : ConfigKeyword -> String
  | Task => "task"
end ConfigKeyword

inductive LiasonKeyword
  | Haha
  deriving BEq
instance : ToString LiasonKeyword where
  toString: LiasonKeyword -> String
  | Haha => sorry
 

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
instance : ToString Keyword where
  toString : Keyword -> String
  | Config (kw) => ToString.toString kw
  | Paradigm (kw) => ToString.toString kw
  | Ty (kw) => ToString.toString kw
  | Stmt (kw) => ToString.toString kw
  | Liason (kw) => ToString.toString kw
  | Main => "main"
  | Use => "use"
  | Mod => "mod"
  | Visual => "visual"
end Keyword

example : ∀ a b : Keyword, a = b ↔ (ToString.toString a) = (ToString.toString b) := by
  intro a b
  apply Iff.intro
  case mp =>
    intro h
    sorry
  case mpr =>
    sorry