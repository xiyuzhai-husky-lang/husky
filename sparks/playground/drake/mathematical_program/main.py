#!/usr/bin/python
from pydrake.solvers import MathematicalProgram
import numpy as np
import matplotlib.pyplot as plt

"""
Solves a simple optimization problem
       min x(0)^2 + x(1)^2
subject to x(0) + x(1) = 1
           x(0) <= x(1)
"""
from pydrake.solvers import Solve

# Set up the optimization problem.
prog = MathematicalProgram()
x = prog.NewContinuousVariables(2)
prog.AddConstraint(x[0] + x[1] == 1)
prog.AddConstraint(x[0] <= x[1])
prog.AddCost(x[0] ** 2 + x[1] ** 2)

# Now solve the optimization problem.
result = Solve(prog)

# print out the result.
print("Success? ", result.is_success())
# Print the solution to the decision variables.
print("x* = ", result.GetSolution(x))
# Print the optimal cost.
print("optimal cost = ", result.get_optimal_cost())
# Print the name of the solver that was called.
print("solver is: ", result.get_solver_id().name())
