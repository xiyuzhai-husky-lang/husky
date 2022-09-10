import Specs.syntax.Text

inductive HuskyTokenKind
  | Decorator
  | Keyword
  | Identifier
  | Special
  | WordOpr
  | WordPattern
  | PrimitiveLiteral
  | Unrecognized
  | IllFormedLiteral

namespace HuskyTokenKind
def as_str : HuskyTokenKind -> String
  | Decorator
  | Keyword
  | Identifier
  | Special
  | WordOpr
  | WordPattern
  | PrimitiveLiteral
  | Unrecognized
  | IllFormedLiteral => sorry
end HuskyTokenKind

structure HuskyToken where
  range: TextRange
  kind : HuskyTokenKind