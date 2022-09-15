inductive ParadigmKeyword
  | Def
  | Func
  | Proc
  deriving DecidableEq

namespace ParadigmKeyword
def kindName : ParadigmKeyword -> String
  | Def => "Def"
  | Func => "Func"
  | Proc => "Proc"

def huskyCode : ParadigmKeyword -> String
  | Def => "def"
  | Func => "func"
  | Proc => "proc"

instance : ToString ParadigmKeyword where
  toString : ParadigmKeyword -> String
  | kw => s!"ParadigmKeyword::{kw.kindName}"
end ParadigmKeyword