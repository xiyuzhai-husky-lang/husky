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
    then lexIdentAux [c] cs
    else if c.isDigit
    then Token.Number::[]
    else Token.Error::[]
termination_by cs.length

def lexIdentAux(partialWord: List Char)(cs: List Char): List Token :=
  match cs with
  | [] => [Token.Ident ⟨partialWord⟩]
  | c::cs =>
    if c.isAlphanum || c=='_'
    then lexIdentAux (partialWord.append [c]) cs
    else lexAux c cs
termination_by cs.length
end

#eval lex "a"
