import HuskyLeanSpecs.word.keyword

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
  | keyword: Keyword -> Word
  | Opr: WordOpr -> Word
  | Decorator: Decorator -> Word
  | Pattern: WordPattern -> Word
  | Identifier