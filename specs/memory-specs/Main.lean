import MemorySpecs

def Set := Nat -> Bool

namespace Set
def empty_set : Set :=
  fun _ => false

def is_empty(set: Set) : Prop := ¬∃ i: Nat, set i

example : is_empty empty_set := sorry
end Set


structure Memory where
  stack: Set
  heap: Set

namespace Memory
def clear (m: Memory): Memory := { m with stack := Set.empty_set, heap := Set.empty_set }
end Memory

structure Heap where
  blocks: Set


namespace Heap
end Heap