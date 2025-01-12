import Mathlib
import Obvious
open Obvious

namespace Example1
def h := by
  have h1 : 1 + 1 = 2 := by obvious
  exact ()
end Example1

namespace Example2
def h (x : ℝ) := by
  have h1 : x ^ 2 ≥ (0 : ℝ) := by obvious
  exact ()
end Example2

namespace Example3
def h (x : ℝ) := by
  have h1 : x ^ 2 + (1 : ℝ) ≥ (2 : ℝ) * x := by obvious
  exact ()
end Example3

namespace Example4
def h := by
  first
  | have h1 : 1 = 1 := by calc
    1 = 1 := by obvious
    _ = 1 := by obvious
  | have h2 : 1 = 1 := by calc
    1 = 1 := by obvious
    _ = 1 := by obvious
  first
  | have h3 : 1 < 2 := by calc
    1 = 1 := by obvious
    _ < 2 := by obvious
  | have h4 : 1 < 2 := by calc
    1 = 1 := by obvious
    _ < 2 := by obvious
  have h5 : 1.0 ≤ 1 := by calc
    1.0 ≤ (1 : ℚ) := by obvious
    _ ≤ 1 := by obvious
  exact ()
end Example4
