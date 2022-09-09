inductive Paradigm
  | LazyFunctional
  | EagerFunctional
  | EagerProcedural

namespace Paradigm
def ParadigmEnumeration := [
  LazyFunctional,
  EagerFunctional,
  EagerProcedural
]
def as_str : Paradigm -> String
  | LazyFunctional => "def"
  | EagerFunctional => "func"
  | EagerProcedural => "proc"
end Paradigm


deriving instance BEq for Paradigm
deriving instance DecidableEq for Paradigm