inductive Token
| Literal
| Ident: (String) -> Token
| Number
| Error
deriving Repr

mutual
def lex: String -> List Token
| ⟨[]⟩  => []
| ⟨c::cs⟩ => lexAux c cs

def lexAux
  (c: Char)
  (cs: List Char): List Token :=
    if c.isAlpha || c=='_'
    then lexIdent c cs
    else if c.isDigit
    then Token.Number::[]
    else Token.Error::[]

def lexIdent(c: Char): List Char -> List Token := lexIdentAux [c]

def lexIdentAux(partialWord: List Char): List Char -> List Token
| [] => [Token.Ident ⟨partialWord⟩]
| c::cs =>
  if c.isAlphanum || c=='_'
  then lexIdentAux (partialWord.append [c]) cs
  else lexAux c cs
end

#eval lex "a"
