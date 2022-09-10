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
instance : ToString Paradigm where
  toString : Paradigm -> String
  | paradigm => paradigm.as_word