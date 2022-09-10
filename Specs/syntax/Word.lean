import Specs.syntax.Word.Keyword
import Specs.syntax.Word.Identifier
import Specs.abstraction.Enumerable

inductive WordOpr
  | And
  | Or
  | As
  | Be
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
  deriving BEq

namespace Decorator
  def toRustVersion : Decorator -> String
    | Pub => "Decorator::Pub"
    | Priviate => "Decorator::Priviate"
    | Async => "Decorator::Async"
    | Static => "Decorator::Static"
  instance : Enumerable Decorator where
    enumeration := [Pub, Priviate, Async, Static]
    hvalid := sorry
end Decorator

inductive WordPattern
  | Some
  | None

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

namespace Word
def toRustVersion : Word -> String
  | Keyword kw => s!"Word::Keyword({kw.toRustVersion})"
  | Opr opr => s!"Word::Opr({opr.toRustVersion})" -- Keyword -> Opr
  | Decorator dec => s!"Word::Decorator({dec.toRustVersion})" -- Decorator
  | Pattern patt => s!"Word::Keyword({patt.toRustVersion})"
  | Identifier ident => s!"Word::Keyword({ident.toRustVersion})"
end Word