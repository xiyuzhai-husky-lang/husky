import Specs.syntax.Word.Keyword.StmtKeyword
import Specs.syntax.Word.Keyword.TyKeyword
import Specs.syntax.Word.Keyword.LiasonKeyword
import Specs.syntax.Word.Keyword.ConfigKeyword
import Specs.syntax.Word.Keyword.ParadigmKeyword

-- type definition

inductive Keyword
  | Config: ConfigKeyword -> Keyword
  | Paradigm: ParadigmKeyword -> Keyword
  | EtherealTerm: TyKeyword -> Keyword
  | Stmt: StmtKeyword -> Keyword
  | Liason: LiasonKeyword -> Keyword
  | Main
  | Use
  | Mod
  | Visual
  deriving DecidableEq

namespace Keyword
  -- method definition

  def kindName : Keyword -> String
    | Config _ => "Config"
    | Paradigm _ => "Paradigm"
    | EtherealTerm _ => "EtherealTerm"
    | Stmt _ => "Stmt"
    | Liason _ => "Liason"
    | Main => "Main"
    | Use => "Use"
    | Mod => "Mod"
    | Visual => "Visual"

  instance : ToString Keyword where
    toString : Keyword -> String
    | Config kw => s!"Keyword::Config({kw})"
    | Paradigm kw => s!"Keyword::Paradigm({kw})"
    | EtherealTerm kw => s!"Keyword::EtherealTerm({kw})"
    | Stmt kw => s!"Keyword::Stmt({kw})"
    | Liason kw => s!"Keyword::Liason({kw})"
    | Main => "Keyword::Main"
    | Use => "Keyword::Use"
    | Mod => "Keyword::Mod"
    | Visual => "visual"

  
  def huskyCode : Keyword -> String
    | Config kw => kw.huskyCode
    | Paradigm kw => kw.huskyCode
    | EtherealTerm kw => kw.huskyCode
    | Stmt kw => kw.huskyCode
    | Liason kw => kw.huskyCode
    | Main => "main"
    | Use => "use"
    | Mod => "mod"
    | Visual => "visual"
end Keyword