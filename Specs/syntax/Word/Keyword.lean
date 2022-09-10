import Specs.syntax.Word.Keyword.StmtKeyword
import Specs.syntax.Word.Keyword.TyKeyword
import Specs.syntax.Word.Keyword.LiasonKeyword
import Specs.syntax.Word.Keyword.ConfigKeyword
import Specs.syntax.Paradigm

-- type definition

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

-- method definition

namespace Keyword
def toRustVersion : Keyword -> String
  | Config (kw) => s!"Keyword::Config({kw.toRustVersion})"
  | Paradigm (kw) => s!"Keyword::Paradigm({kw.toRustVersion})"
  | Ty (kw) => s!"Keyword::Ty({kw.toRustVersion})"
  | Stmt (kw) => s!"Keyword::Stmt({kw.toRustVersion})"
  | Liason (kw) => s!"Keyword::Liason({kw.toRustVersion})"
  | Main => "Keyword::Main"
  | Use => "Keyword::Use"
  | Mod => "Keyword::Mod"
  | Visual => "visual"
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

-- proofs

example : ∀ a b : Keyword, (a = b) = ((ToString.toString a) = (ToString.toString b))
  | Config (kw1), Config (kw2) => by simp
  | Config (ConfigKeyword.Task), Paradigm (Paradigm.LazyFunctional) => by simp
  | Config (ConfigKeyword.Task), Paradigm (Paradigm.EagerFunctional) => by simp
  | Config (ConfigKeyword.Task), Paradigm (Paradigm.EagerProcedural) => by simp
  | _, _ => sorry
end Keyword