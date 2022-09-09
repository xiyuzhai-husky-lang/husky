inductive Paradigm
  | LazyFunctional
  | EagerFunctional
  | EagerProcedural
  deriving BEq

namespace Paradigm
def ParadigmEnumeration := [
  LazyFunctional,
  EagerFunctional,
  EagerProcedural
]
def as_word : Paradigm -> String
  | LazyFunctional => "def"
  | EagerFunctional => "func"
  | EagerProcedural => "proc"
end Paradigm