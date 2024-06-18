inductive PrefixOpr
| Not
deriving Repr

inductive SuffixOpr
deriving Repr

inductive BinaryOpr
| Add
| Sub
| Mul
| Div
deriving Repr

inductive Opr
| Prefix: PrefixOpr -> Opr
| Suffix: SuffixOpr -> Opr
| Binary: BinaryOpr -> Opr
deriving Repr

inductive Delimiter
| Par
| Box
| Curl
deriving Repr

inductive Token
| Literal
| Ident: (String) -> Token
| Number
| Error
| Opr: Opr -> Token
| LeftDelimiter: Delimiter -> Token
| RightDelimiter: Delimiter -> Token
deriving Repr



mutual
def lexInner: List Char -> List Token
| []=> []
| c::cs => lexAux c cs

def lexAux
  (c: Char)
  (cs: List Char): List Token :=
    if c.isAlpha || c=='_'
    then lexIdentAux [c] cs
    else if c.isDigit
    then lexNumberAux [c] cs
    else match c with
      | ' ' => lexInner cs
      | '+' => (Token.Opr (Opr.Binary BinaryOpr.Add))::(lexInner cs)
      | '-' => (Token.Opr (Opr.Binary BinaryOpr.Sub))::(lexInner cs)
      | '*' => (Token.Opr (Opr.Binary BinaryOpr.Mul))::(lexInner cs)
      | '/' => (Token.Opr (Opr.Binary BinaryOpr.Div))::(lexInner cs)
      | '!' => (Token.Opr (Opr.Prefix PrefixOpr.Not))::(lexInner cs)
      | '(' => (Token.LeftDelimiter Delimiter.Par)::(lexInner cs)
      | ')' => (Token.RightDelimiter Delimiter.Par)::(lexInner cs)
      | _ => [Token.Error]

def lexIdentAux(partialWord: List Char)(cs: List Char): List Token :=
  match cs with
  | [] => [Token.Ident ⟨partialWord⟩]
  | c::cs =>
    if c.isAlphanum || c=='_'
    then lexIdentAux (partialWord.append [c]) cs
    else (Token.Ident ⟨partialWord⟩)::(lexAux c cs)

def lexNumberAux(partialWord: List Char)(cs: List Char): List Token :=
  match cs with
  | [] => [Token.Ident ⟨partialWord⟩]
  | c::cs =>
    if c.isDigit
    then lexIdentAux (partialWord.append [c]) cs
    else (Token.Ident ⟨partialWord⟩)::(lexAux c cs)
end

def lex: String -> List Token
| ⟨[]⟩  => []
| ⟨c::cs⟩ => lexAux c cs

#eval lex "a"
#eval lex " a"
#eval lex "a b"
#eval lex "1"
#eval lex "-"
#eval lex "!"
#eval lex "!a"
#eval lex "! a"
#eval lex "(a"
#eval lex "()"
