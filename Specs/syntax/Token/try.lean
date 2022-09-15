inductive SimpleToken where
  | A
  | BB
  | Custom : Char -> SimpleToken

namespace SimpleToken
def code : SimpleToken -> List Char
  | A => "A".toList
  | BB => "BB".toList
  | Custom c => [c]

def writeCode : List SimpleToken -> List Char
  | [] => []
  | a::as => a.code.append (writeCode as)

def parseCode : List Char -> List SimpleToken
  | [] => []
  | 'A'::as => [A].append (parseCode as)
  | 'B'::'B'::as => [BB].append (parseCode as)
  | c::as => [Custom c].append (parseCode as)

theorem t : ∀ tokens : List SimpleToken, tokens = parseCode (writeCode tokens)
  | [] => rfl
  | A::as => by
    simp[writeCode, code, parseCode]
    apply t as
  | BB::as => by
    simp[writeCode, code, parseCode]
    apply t as
  | (Custom c)::as => by
    simp[writeCode, code, parseCode]
    -- apply t as
    sorry
end SimpleToken