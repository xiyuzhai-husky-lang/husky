import Specs.syntax.Word.Keyword
import Specs.syntax.Word.Identifier

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
  | Keyword: Keyword -> Word
  | Opr: WordOpr -> Word
  | Decorator: Decorator -> Word
  | Pattern: WordPattern -> Word
  | Identifier: Identifier -> Word

namespace Word
def as_str : Word -> String
  | keyword (a) => a.as_str
  | _ => sorry
end Word