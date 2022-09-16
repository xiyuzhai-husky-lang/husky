import Specs.syntax.Word
import Specs.syntax.PrimitiveLiteralData
import Specs.syntax.Token.SpecialToken -- mod SpecialToken; use SpecialToken


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
  deriving DecidableEq

namespace HuskyTokenKind
def kindName : HuskyTokenKind -> String
  | Decorator _ => "Decorator"
  | Keyword _ => "Keyword"
  | Identifier _ => "Identifier"
  | Special _ => "Special"
  | WordOpr _ => "WordOpr"
  | WordPattern _ => " WordPattern"
  | PrimitiveLiteral _ => "PrimitiveLiteral"
  | Unrecognized _ => "Unrecognized"
  | IllFormedLiteral _ => "IllFormedLiteral"

def huskyCode : HuskyTokenKind -> String
  | Decorator dec => dec.huskyCode
  | Keyword kw => kw.huskyCode
  | Identifier ident => ident.huskyCode
  | Special special => special.huskyCode
  | WordOpr opr => opr.huskyCode
  | WordPattern patt => patt.huskyCode
  | PrimitiveLiteral data => data.huskyCode
  | Unrecognized c => [c].asString
  | IllFormedLiteral data => data.huskyCode
end HuskyTokenKind

structure HuskyToken where
  range: TextRange
  kind : HuskyTokenKind