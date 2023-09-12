import Specs.abstraction.Enumerable

inductive SpecialToken
  | LAngle             -- <
  | Leq                -- <=
  | RAngle             -- >
  | Geq                -- >=
  | Neq                -- !=
  | DeriveAssign       -- ?=
  | Eq                 -- ==
  | Shl                -- <<
  | LCurl              -- {
  | RCurl              -- }
  | Lbox               -- [
  | Rbox               --]
  | LPar               -- (
  | RPar               --)
  | Add                -- +
  | SubOrMinus         -- -
  | Star               -- *
  | Div                -- /
  | Power              -- **
  | And                -- &&
  | DoubleVertical     -- ||
  | BitNot             -- ~
  | Modulo             -- %
  | FieldAccess        -- .
  | LightArrow         -- ->
  | HeavyArrow         -- =>
  | DoubleColon        -- ::
  | Colon              -- :
  | Comma              -- ,
  | Ambersand          -- &
  | Incr               -- ++
  | Decr               -- --
  | Vertical           -- |
  | Assign             -- =
  | AddAssign          -- +=
  | SubAssign          -- -=
  | MulAssign          -- *=
  | DivAssign          -- /=
  | BitAndAssign       -- &=
  | BitOrAssign        -- |=
  | Exclamation        -- !
  | DoubleExclamation  -- !!
  | Semicolon          -- ;
  | XmlKet             -- />
  | At                 -- @
  | Question       -- ?
  deriving DecidableEq

namespace SpecialToken
  def toRustVersion : SpecialToken -> String
    | LAngle => "SpecialToken::LAngle"
    | Leq => "SpecialToken:Leq"
    | RAngle => "SpecialToken::RAngle"
    | Geq => "SpecialToken::Geq"
    | Neq => "SpecialToken::Neq"
    | DeriveAssign => "SpecialToken::DeriveAssign"
    | Eq => "SpecialToken::Eq"
    | Shl => "SpecialToken::Shl"
    | LCurl => "SpecialToken::Bra(Bracket::Curl)"
    | RCurl => "SpecialToken::Ket(Bracket::Curl)"
    | Lbox => "SpecialToken::Bra(Bracket::Box)"
    | Rbox => "SpecialToken::Ket(Bracket::Box)"  
    | LPar => "SpecialToken::Bra(Bracket::Par)"
    | RPar => "SpecialToken::Ket(Bracket::Par)"
    | Add => "SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Add))"
    | SubOrMinus => "SpecialToken::SubOrMinus"
    | Star => "SpecialToken::Star"
    | Div => "SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Div))"
    | Power => "SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Power))"
    | And => "SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::And))"
    | DoubleVertical => "SpecialToken::DoubleVertical"
    | BitNot => "SpecialToken::BitNot"
    | Modulo => "SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::RemEuclid))"
    | FieldAccess => "SpecialToken::FieldAccess"
    | LightArrow => "SpecialToken::BinaryOpr(BinaryOpr::Curry)"
    | HeavyArrow => "SpecialToken::BinaryOpr(BinaryOpr::HeavyArrow)"
    | DoubleColon => "SpecialToken::BinaryOpr(BinaryOpr::ScopeResolution)"
    | Colon => "SpecialToken::Colon"
    | Comma => "SpecialToken::Comma"
    | Ambersand => "SpecialToken::Ambersand"
    | Incr => "SpecialToken::Incr"
    | Decr => "SpecialToken::Decr"
    | Vertical => "SpecialToken::Vertical"
    | Assign => "SpecialToken::BinaryOpr(BinaryOpr::Assign(None))"
    | AddAssign => "SpecialToken::AddAssign"
    | SubAssign => "SpecialToken::SubAssign"
    | MulAssign => "SpecialToken::MulAssign"
    | DivAssign => "SpecialToken::DivAssign"
    | BitAndAssign => "SpecialToken::BitAndAssign"
    | BitOrAssign => "SpecialToken::BitOrAssign"
    | Exclamation => "SpecialToken::Exclamation"
    | DoubleExclamation => "SpecialToken::DoubleExclamation"
    | Semicolon => "SpecialToken::Semicolon"
    | XmlKet => "SpecialToken::XmlKet"
    | At => "SpecialToken::At"
    | Question => "SpecialToken::Question"

instance : Enumerable SpecialToken where
  enumeration := [
    LAngle,
    Leq,
    RAngle,
    Geq,
    Neq,
    DeriveAssign,
    Eq,
    Shl,
    LCurl,
    RCurl,
    Lbox,
    Rbox,
    LPar,
    RPar,
    Add,
    SubOrMinus,
    Star,
    Div,
    Power,
    And,
    DoubleVertical,
    BitNot,
    Modulo,
    FieldAccess,
    LightArrow,
    HeavyArrow,
    DoubleColon,
    Colon,
    Comma,
    Ambersand,
    Incr,
    Decr,
    Vertical,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    BitAndAssign,
    BitOrAssign,
    Exclamation,
    DoubleExclamation,
    Semicolon,
    XmlKet,
    At,
    Question
  ]
  hvalid := by
    apply And.intro
    rfl
    intro a
    cases a <;> rfl

def huskyCode : SpecialToken -> String
  | LAngle => "<"
  | Leq => "<="
  | RAngle => ">"
  | Geq => ">="
  | Neq => "!="
  | DeriveAssign => "?="
  | Eq => "=="
  | Shl => "<<"
  | LCurl => "{"
  | RCurl => "}"
  | Lbox => "["
  | Rbox => "]"
  | LPar => "("
  | RPar => ")"
  | Add => "+"
  | SubOrMinus => "-"
  | Star => "*"
  | Div => "/"
  | Power => "**"
  | And => "&&"
  | DoubleVertical => "||"
  | BitNot => "~"
  | Modulo => "%"
  | FieldAccess => "."
  | LightArrow => "->"
  | HeavyArrow => "=>"
  | DoubleColon => "::"
  | Colon => ":"
  | Comma => ""
  | Ambersand => "&"
  | Incr => "++"
  | Decr => "--"
  | Vertical => "|"
  | Assign => "="
  | AddAssign => "+="
  | SubAssign => "-="
  | MulAssign => "*="
  | DivAssign => "/="
  | Exclamation => "!"
  | BitOrAssign => "|="
  | BitAndAssign => "&="
  | DoubleExclamation => "!!"
  | Semicolon => ";"
  | XmlKet => "/>"
  | At => "@"
  | Question => "?"
end SpecialToken