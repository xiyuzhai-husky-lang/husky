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
  have h1 : x ^ 2 ≥ (0 : ℝ) := by obvious
  exact ()
end Example2

namespace Example3
def h(a : ℝ)(b : ℝ) := by
  have h1 : a + b = b + a := by obvious
  exact ()
end Example3

namespace Example4
def h(x : ℝ) := by
  have h1 : x ^ 2 ≥ (0 : ℝ) := by obvious
  exact ()
end Example4

namespace Example5
def h(x : ℝ)(h1 : x > (0 : ℝ)) := by
  have h2 : x + (1 : ℝ) / x - (2 : ℝ) = (x ^ 2 + (1 : ℝ) - (2 : ℝ) * x) / x := by obvious
  have h3 : (x ^ 2 + (1 : ℝ) - (2 : ℝ) * x) / x = (x - (1 : ℝ)) ^ 2 / x := by obvious
  have h4 : x + (1 : ℝ) / x - (2 : ℝ) = (x - (1 : ℝ)) ^ 2 / x := by obvious
  have h5 : (x - (1 : ℝ)) ^ 2 / x ≥ (0 : ℝ) := by obvious
  have h6 : x + (1 : ℝ) / x - (2 : ℝ) ≥ (0 : ℝ) := by obvious
  have h7 : x + (1 : ℝ) / x ≥ (2 : ℝ) := by obvious
  exact ()
end Example5

namespace Example6
def h(x : ℝ)(h1 : x > (0 : ℝ)) := by
  have h2 : x + (1 : ℝ) / x - (2 : ℝ) = (x ^ 2 + (1 : ℝ) - (2 : ℝ) * x) / x := by obvious
  have h3 : (x ^ 2 + (1 : ℝ) - (2 : ℝ) * x) / x = (x - (1 : ℝ)) ^ 2 / x := by obvious
  have h4 : x + (1 : ℝ) / x - (2 : ℝ) = (x - (1 : ℝ)) ^ 2 / x := by obvious
  have h5 : (x - (1 : ℝ)) ^ 2 / x ≥ (0 : ℝ) := by obvious
  have h6 : x + (1 : ℝ) / x - (2 : ℝ) ≥ (0 : ℝ) := by obvious
  have h7 : x + (1 : ℝ) / x ≥ (2 : ℝ) := by obvious
  exact ()
end Example6

namespace Example7
def h(x : ℝ) := by
  have h1 : x ^ 2 + (1 : ℝ) - (2 : ℝ) * x = (x - (1 : ℝ)) ^ 2 := by obvious
  have h2 : (x - (1 : ℝ)) ^ 2 ≥ (0 : ℝ) := by obvious
  have h3 : x ^ 2 + (1 : ℝ) - (2 : ℝ) * x ≥ (0 : ℝ) := by obvious
  have h4 : x ^ 2 + (1 : ℝ) ≥ (2 : ℝ) * x := by obvious
  exact ()
end Example7

namespace Example8
def h(x : ℝ)(h1 : x > (0 : ℝ)) := by
  have h2 : x + (1 : ℝ) - (2 : ℝ) * √ x = (√ x - (1 : ℝ)) ^ 2 := by obvious
  have h3 : (√ x - (1 : ℝ)) ^ 2 ≥ (0 : ℝ) := by obvious
  have h4 : x + (1 : ℝ) - (2 : ℝ) * √ x ≥ (0 : ℝ) := by obvious
  have h5 : x + (1 : ℝ) ≥ (2 : ℝ) * √ x := by obvious
  exact ()
end Example8

namespace Example9
def h(x : ℝ)(h1 : x > (0 : ℝ))(y : ℝ)(h2 : y > (0 : ℝ)) := by
  have h3 : (1 : ℝ) / x + (1 : ℝ) / y - (4 : ℝ) / (x + y) = (y * (x + y) + x * (x + y) - (4 : ℝ) * x * y) / (x * y * (x + y)) := by obvious
  have h4 : (y * (x + y) + x * (x + y) - (4 : ℝ) * x * y) / (x * y * (x + y)) = (y * x + y ^ 2 + x ^ 2 + y * x - (4 : ℝ) * x * y) / (x * y * (x + y)) := by obvious
  have h5 : (y * x + y ^ 2 + x ^ 2 + y * x - (4 : ℝ) * x * y) / (x * y * (x + y)) = (y ^ 2 + x ^ 2 - (2 : ℝ) * x * y) / (x * y * (x + y)) := by obvious
  have h6 : (y ^ 2 + x ^ 2 - (2 : ℝ) * x * y) / (x * y * (x + y)) = (x - y) ^ 2 / (x * y * (x + y)) := by obvious
  have h7 : (x - y) ^ 2 / (x * y * (x + y)) ≥ (0 : ℝ) := by obvious
  have h8 : (1 : ℝ) / x + (1 : ℝ) / y - (4 : ℝ) / (x + y) ≥ (0 : ℝ) := by obvious
  have h9 : (1 : ℝ) / x + (1 : ℝ) / y ≥ (4 : ℝ) / (x + y) := by obvious
  exact ()
end Example9

namespace Example10
def h(a : ℝ)(h1 : a > (0 : ℝ))(b : ℝ)(h2 : b > (0 : ℝ)) := by
  have h3 : a / b + b / a - (2 : ℝ) = (a ^ 2 + b ^ 2 - (2 : ℝ) * a * b) / (a * b) := by obvious
  have h4 : (a ^ 2 + b ^ 2 - (2 : ℝ) * a * b) / (a * b) = (a - b) ^ 2 / (a * b) := by obvious
  have h5 : (a - b) ^ 2 / (a * b) ≥ (0 : ℝ) := by obvious
  have h6 : a / b + b / a - (2 : ℝ) ≥ (0 : ℝ) := by obvious
  have h7 : a / b + b / a ≥ (2 : ℝ) := by obvious
  exact ()
end Example10

namespace Example11
def h(x : ℝ)(y : ℝ) := by
  have h1 : x ^ 2 + y ^ 2 - (2 : ℝ) * x * y = (x - y) ^ 2 := by obvious
  have h2 : (x - y) ^ 2 ≥ (0 : ℝ) := by obvious
  have h3 : x ^ 2 + y ^ 2 - (2 : ℝ) * x * y ≥ (0 : ℝ) := by obvious
  have h4 : x ^ 2 + y ^ 2 ≥ (2 : ℝ) * x * y := by obvious
  exact ()
end Example11
