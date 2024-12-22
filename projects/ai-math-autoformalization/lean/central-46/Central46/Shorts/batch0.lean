import Mathlib
import Obvious

open Obvious

namespace Example1
def h := by
  have h1 : 1 + 1 = 2 := by obvious
  exact ()
end Example1

namespace Example2
def h(x : ℝ) := by
  have h1 : x ^ 2 ≥ 0 := by obvious
  exact ()
end Example2

namespace Example3
def h(x : ℝ) := by
  have h1 : x ^ 2 + 1 ≥ 2 * x := by obvious
  exact ()
end Example3

namespace Example4
def h := by
  have h1 : 1 = 1 := by calc
      1 = 1 := sorry
    _ = 1 := sorry
  have h2 : 1 < 2 := by calc
      1 = 1 := sorry
    _ < 2 := sorry
  have h3 : 1.0 ≤ 1 := by calc
      1.0 ≤ 1 := sorry
    _ ≤ 1 := sorry
  exact ()
end Example4
