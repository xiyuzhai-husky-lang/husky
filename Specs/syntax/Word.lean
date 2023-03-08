import Specs.syntax.Word.Keyword
import Specs.syntax.Word.Ident
import Specs.abstraction.Enumerable

inductive WordOpr
  | And
  | Or
  | As
  | Be
  deriving DecidableEq
namespace WordOpr
instance : ToString WordOpr where
  toString : WordOpr -> String
  | And => "WordOpr::And"
  | Or => "WordOpr::Or"
  | As => "WordOpr::As"
  | Be => "WordOpr::Be"

def huskyCode : WordOpr -> String
  | And => "and"
  | Or => "or"
  | As => "as"
  | Be => "be"
end WordOpr

inductive Decorator
  | Pub
  | Private
  | Async
  | Static
  deriving DecidableEq

namespace Decorator
  instance : ToString Decorator where
    toString : Decorator -> String
    | Pub => "Decorator::Pub"
    | Private => "Decorator::Priviate"
    | Async => "Decorator::Async"
    | Static => "Decorator::Static"
  def huskyCode : Decorator -> String
    | Pub => "pub"
    | Private => "private"
    | Async => "async"
    | Static => "static"
  instance : Enumerable Decorator where
    enumeration := [Pub, Private, Async, Static]
    hvalid := by
      apply And.intro
      apply rfl
      intro a
      cases a with
      | _ => rfl
end Decorator

inductive WordPattern
  | Some
  | None
  deriving DecidableEq

namespace WordPattern
  instance : ToString WordPattern where
  toString : WordPattern -> String
    | Some => "WordPattern::Some"
    | None => "WordPattern::None"
  
  def huskyCode : WordPattern -> String
    | Some => "some"
    | None => "none"
end WordPattern

-- inductive Word
--   | Keyword: Keyword -> Word
--   | Opr: WordOpr -> Word
--   | Decorator: Decorator -> Word
--   | Pattern: WordPattern -> Word
--   | Ident: Ident -> Word
--   deriving DecidableEq

-- namespace Word
-- instance : ToString Word where
--   toString : Word -> String
--   | Keyword kw => s!"Word::Keyword({kw})"
--   | Opr opr => s!"Word::Opr({opr})" -- Keyword -> Opr
--   | Decorator dec => s!"Word::Decorator({dec})" -- Decorator
--   | Pattern patt => s!"Word::Keyword({patt})"
--   | Ident ident => s!"Word::Ident({ident})"
-- end Word