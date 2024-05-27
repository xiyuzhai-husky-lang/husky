def hello := "world"

structure SuperComputationGraph0(size: Nat) where
  incoming_nodes: (Fin size) -> List (Fin size)
  constructors: (Fin size) -> List (Fin size)
