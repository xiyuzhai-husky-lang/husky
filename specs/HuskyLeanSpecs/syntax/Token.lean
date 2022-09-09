import HuskyLeanSpecs.Text

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
def to_string : HuskyTokenKind -> String
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