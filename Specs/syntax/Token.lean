import Specs.syntax.Word
import Specs.syntax.LiteralToken
import Specs.syntax.Token.SpecialToken -- mod SpecialToken; use SpecialToken

inductive TokenKind
  | Decorator : Decorator -> TokenKind
  | Keyword : Keyword -> TokenKind
  | Ident : Ident -> TokenKind
  | Special : SpecialToken -> TokenKind
  | WordOpr : WordOpr -> TokenKind
  | WordPattern : WordPattern-> TokenKind
  | PrimitiveLiteral : LiteralToken -> TokenKind
  | Unrecognized : Char -> TokenKind
  | IllFormedLiteral : LiteralToken -> TokenKind
  deriving DecidableEq

namespace TokenKind
def kindName : TokenKind -> String
  | Decorator _ => "Decorator"
  | Keyword _ => "Keyword"
  | Ident _ => "Ident"
  | Special _ => "Special"
  | WordOpr _ => "WordOpr"
  | WordPattern _ => " WordPattern"
  | PrimitiveLiteral _ => "PrimitiveLiteral"
  | Unrecognized _ => "Unrecognized"
  | IllFormedLiteral _ => "IllFormedLiteral"

def huskyCode : TokenKind -> String
  | Decorator dec => dec.huskyCode
  | Keyword kw => kw.huskyCode
  | Ident ident => ident.huskyCode
  | Special special => special.huskyCode
  | WordOpr opr => opr.huskyCode
  | WordPattern patt => patt.huskyCode
  | PrimitiveLiteral data => data.huskyCode
  | Unrecognized c => [c].asString
  | IllFormedLiteral data => data.huskyCode
end TokenKind

structure HuskyToken where
  range: TextRange
  kind : TokenKind