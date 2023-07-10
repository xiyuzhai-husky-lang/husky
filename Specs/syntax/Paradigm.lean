inductive Paradigm
  | LazyFunctional
  | EagerFunctional
  | EagerProcedural
  deriving DecidableEq

namespace Paradigm
def toRustVersion : Paradigm -> String
  | LazyFunctional => "Paradigm::LazyFunctional"
  | EagerFunctional => "Paradigm::EagerFunctional"
  | EagerProcedural => "Paradigm::EagerProcedural"
def ParadigmEnumeration := [
  LazyFunctional,
  EagerFunctional,
  EagerProcedural
]
def as_coword : Paradigm -> String
  | LazyFunctional => "def"
  | EagerFunctional => "func"
  | EagerProcedural => "proc"
end Paradigm
instance : ToString Paradigm where
  toString : Paradigm -> String
  | paradigm => paradigm.as_coword


