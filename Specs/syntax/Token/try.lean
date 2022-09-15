structure ValidChar where
  c : Char
  h : c != ' '

structure SimpleToken where
  value : List ValidChar

namespace SimpleToken
def toChars (t : SimpleToken) : List Char :=
  t.value.map fun vc => vc.c
end SimpleToken

def writeCode (tokens : List SimpleToken) : List Char :=
  [' '].intercalate (tokens.map fun token => token.toChars)

def parseCodeAux : List Char -> List ValidChar -> List SimpleToken
  | [], pref => [{ value := pref : SimpleToken }]
  | ' ' :: as, pref => [{ value := pref : SimpleToken }] ++ (parseCodeAux as [])
  | c :: as, pref => parseCodeAux as (pref.concat {
    c,
    h := by
      sorry
    :ValidChar
  })

def parseCode (chars : List Char) : List SimpleToken := sorry

theorem parsing_is_correct (tokens : List SimpleToken) : parseCode (writeCode tokens) = tokens := by
  sorry

structure NonZeroNat where
  raw : Nat
  h : raw > 0

def fromRaw : Nat -> Option NonZeroNat
  | 0 => none
  | raw => some {
    raw,
    h := sorry
    : NonZeroNat
  }