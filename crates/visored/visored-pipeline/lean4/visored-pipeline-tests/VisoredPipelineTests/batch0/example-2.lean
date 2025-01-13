import Mathlib
syntax "attack" : tactic

macro_rules
| `(tactic| attack) => `(tactic|
  first
  | simp; done
  | ring; done
  | ring_nf; rw [Real.sq_sqrt]; ring; all_goals attack; done
  | nlinarith; done
  | apply sq_nonneg; all_goals attack; done
  | apply div_nonneg; all_goals attack; done
  | field_simp; ring; done
  | linarith; done
)

macro "obvious": tactic =>`(tactic|
  first
  | attack; done
  | congr; all_goals attack; done
  | gcongr; all_goals attack; done
  | fail "Could not prove this goal automatically. It might not be as obvious as you think!"
)

macro "in_set" : term => `(true)

def h (x : ℝ) (h1 : x ≥ (0 : ℝ)) (y : ℝ) (h2 : y ≥ (0 : ℝ)) : (x + y) / (2 : ℝ) ≥ √ (x * y) := by
  have h3 : (√ x - √ y) ^ 2 = √ x ^ 2 - (2 : ℝ) * √ x * √ y + √ y ^ 2 := by obvious
  have h4 : √ x ^ 2 - (2 : ℝ) * √ x * √ y + √ y ^ 2 = x - (2 : ℝ) * √ (x * y) + y := by obvious
  have h5 : x - (2 : ℝ) * √ (x * y) + y ≥ (0 : ℝ) := by obvious
  have h6 : (√ x - √ y) ^ 2 ≥ (0 : ℝ) := by obvious
  have h7 : x + y ≥ (2 : ℝ) * √ (x * y) := by obvious
  have h8 : (x + y) / (2 : ℝ) ≥ √ (x * y) := by obvious
  obvious
