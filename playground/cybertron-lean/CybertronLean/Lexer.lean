inductive Token
| Literal

def lex: String -> Token
| _ => Token.Literal
