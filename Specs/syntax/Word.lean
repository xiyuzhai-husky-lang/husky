import Specs.syntax.Word.Keyword
import Specs.syntax.Word.Identifier
import Specs.abstraction.Enumerable

inductive WordOpr
  | And
  | Or
  | As
  | Be
  deriving DecidableEq
namespace WordOpr
def toRustVersion : WordOpr -> String
  | And => "WordOpr::And"
  | Or => "WordOpr::Or"
  | As => "WordOpr::As"
  | Be => "WordOpr::Be"
end WordOpr

inductive Decorator
  | Pub
  | Priviate
  | Async
  | Static
  deriving DecidableEq

namespace Decorator
  def toRustVersion : Decorator -> String
    | Pub => "Decorator::Pub"
    | Priviate => "Decorator::Priviate"
    | Async => "Decorator::Async"
    | Static => "Decorator::Static"
  def huskyCode : Decorator -> String
    | Pub => "pub"
    | Priviate => "private"
    | Async => "async"
    | Static => "static"
  instance : Enumerable Decorator where
    enumeration := [Pub, Priviate, Async, Static]
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
  def toRustVersion : WordPattern -> String
    | Some => "WordPattern::Some"
    | None => "WordPattern::None"
end WordPattern

inductive Word
  | Keyword: Keyword -> Word
  | Opr: WordOpr -> Word
  | Decorator: Decorator -> Word
  | Pattern: WordPattern -> Word
  | Identifier: Identifier -> Word
  deriving DecidableEq

namespace Word
def toRustVersion : Word -> String
  | Keyword kw => s!"Word::Keyword({kw.toRustVersion})"
  | Opr opr => s!"Word::Opr({opr.toRustVersion})" -- Keyword -> Opr
  | Decorator dec => s!"Word::Decorator({dec.toRustVersion})" -- Decorator
  | Pattern patt => s!"Word::Keyword({patt.toRustVersion})"
  | Identifier ident => s!"Word::Keyword({ident.toRustVersion})"
end Word