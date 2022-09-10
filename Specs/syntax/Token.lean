import Specs.syntax.Text
import Specs.syntax.Word

inductive HuskyTokenKind
  | Decorator : Decorator -> HuskyTokenKind
  | Keyword : Keyword -> HuskyTokenKind
  | Identifier : Identifier -> HuskyTokenKind
  | Special : SpecialToken -> HuskyTokenKind
  | WordOpr : WordOpr -> HuskyTokenKind
  | WordPattern : WordPattern-> HuskyTokenKind
  | PrimitiveLiteral : PrimitiveLiteralData -> HuskyTokenKind
  | Unrecognized : Char -> HuskyTokenKind
  | IllFormedLiteral : PrimitiveLiteralData -> HuskyTokenKind

namespace HuskyTokenKind
def as_str : HuskyTokenKind -> String
  | _ => sorry
end HuskyTokenKind

structure HuskyToken where
  range: TextRange
  kind : HuskyTokenKind