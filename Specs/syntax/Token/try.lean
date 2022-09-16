structure ValidChar where
  c : Char
  h : c != ' '

structure SimpleToken where
  chars : List ValidChar
  h : chars.length > 0

namespace SimpleToken
def toChars (t : SimpleToken) : List Char :=
  t.chars.map fun vc => vc.c

def fromValidChars (chars : List ValidChar) : List SimpleToken :=
  if h : chars.length > 0 then
    [{ chars, h : SimpleToken }]
  else
    []
end SimpleToken

def writeCode (tokens : List SimpleToken) : List Char :=
  [' '].intercalate (tokens.map fun token => token.toChars)

def parseCodeAux : List Char -> List ValidChar -> List SimpleToken
  | [], pref => SimpleToken.fromValidChars pref
  | c :: as, pref => 
    if h : c != ' ' then
      parseCodeAux as (pref.concat { c, h : ValidChar })
    else
      SimpleToken.fromValidChars pref ++ (parseCodeAux as [])

def parseCode (chars : List Char) : List SimpleToken := parseCodeAux chars []

theorem parsingIsCorrectAux : (tokens : List SimpleToken)  -> parseCode (writeCode tokens) = tokens
  | [] => by
    simp[writeCode]
    simp[List.map]
    simp[List.intercalate]
    simp[List.intersperse]
    simp[List.join]
    simp[parseCode]
    simp[parseCodeAux]
    simp[SimpleToken.fromValidChars]
  | token::tokens => by
    simp[parseCode]
    simp[writeCode]
    simp[List.map]
    simp[SimpleToken.toChars]
    simp[List.map]
    simp[parseCodeAux]
    simp[parseCodeAux]
    sorry