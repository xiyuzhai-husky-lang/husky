import Specs

def genSpecBook : IO Unit := IO.FS.writeFile "docs/specs.md" "haha"

def genRustTests : IO Unit := do
  return

def main : IO Unit :=
  do
    genSpecBook
    genRustTests