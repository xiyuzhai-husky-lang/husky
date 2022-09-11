import Specs.syntax.Text
import Specs.syntax.Word
import Specs.syntax.Token.SpecialToken

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
def husky_code : HuskyTokenKind -> String
  | Decorator dec => dec.husky_code
  | Keyword kw => kw.husky_code
  | Identifier ident => ident.husky_code
  | Special special => special.husky_code
  | WordOpr opr => opr..husky_code
  | WordPattern patt => patt.husky_code
  | PrimitiveLiteral data => data.husky_code
  | Unrecognized c => c.husky_code
  | IllFormedLiteral data => data.husky_code
end HuskyTokenKind

structure HuskyToken where
  range: TextRange
  kind : HuskyTokenKind