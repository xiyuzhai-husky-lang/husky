import Mathlib
import Obvious

open Obvious

def h := by
  have h1 : 1 + 1 = 2 := by obvious
  exact ()

def h2 := by
  have h3 : x ^ 2 ≥ 0 := by obvious
  exact ()

def h4 := by
  have h5 : x ^ 2 + 1 ≥ 2 * x := by obvious
  exact ()

def h6 := by
  calc
      1 = 1 := sorry
    _ = 1 := sorry
  calc
      1 = 1 := sorry
    _ < 2 := sorry
  calc
      1.0 ≤ 1 := sorry
    _ ≤ 1 := sorry
  exact ()