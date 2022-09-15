import Specs
import Init.System.IO

def genSpecBook : IO Unit := sorry

def genRustTests : IO Unit := sorry

def main : IO Unit :=
  do
    genSpecBook
    genRustTests