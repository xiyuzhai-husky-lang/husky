import Mathlib

macro "term_trivial": tactic =>`(tactic|
  first
  | simp; done
  | ring; done
  | ring_nf; done
  | linarith; done
  | fail "Could not prove this goal automatically. Afterall, this is an ad hoc implementation."
)

macro "term_equivalent": tactic =>`(tactic|
  first
  | simp; done
  | ring; done
  | ring_nf; done
  | linarith; done
  | fail "Could not prove this goal automatically. Afterall, this is an ad hoc implementation."
)

namespace Example1
def h := by
  have h1 : 0 = 0 := by term_trivial
  exact ()
end Example1

namespace Example2
def h := by
  have h1 : 1 + 1 = 2 := by term_trivial
  exact ()
end Example2

namespace Example3
def h := by
  have h1 : 0 < 1 := by term_trivial
  exact ()
end Example3

namespace Example4
def h := by
  have h1 : 0 ≠ 1 := by term_trivial
  exact ()
end Example4

namespace Example5
def h(x : ℝ) := by
  have h1 : x = x := by term_trivial
  exact ()
end Example5

namespace Example6
def h(x : ℝ) := by
  have h1 : x - x = 0 := by term_trivial
  exact ()
end Example6

namespace Example7
def h(x : ℝ) := by
  have h1 : x + x = 2 * x := by term_trivial
  exact ()
end Example7

namespace Example8
def h(x : ℝ) := by
  have h1 : x ^ 2 ≥ 0 := by apply sq_nonneg
  exact ()
end Example8

namespace Example9
def h(x : ℝ)(h1 : x ≥ 1) := by
  have h2 : x - 1 ≥ 0 := by term_equivalent
  exact ()
end Example9

