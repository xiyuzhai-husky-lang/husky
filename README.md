# The Husky Programming Language (Work In Progress)

The Husky programming language is designed for next generation computing, a vast generalization of current AI and software.

It combines strengths from various languages in different domains, like Rust, C/C++, python, Lean4 so that it serves as an amazing tool with a **uniformed** and much more **ergonomic** developing experience for all the following tasks

- system level programming like Rust and C/C++ and Zig
- web/native frontend development like JavaScript/TypeScript
- parser/compiler like Ocaml/Haskell
- game development
- formal verification and theorem proving like Coq/Lean
- machine learning and data science as we know it today like python
- IDE extension client like JavaScript/TypeScript
- scientific computing like Mathematica, python, Matlab and Julia

But even more importantly, it is **powerful enough for many difficult tasks which haven't yet been seen as programmable at all**, including

- fully explainable model image classification that is as accurate as deep learning with much better efficiency and robustness, see [vision a programming approach](impress/vision_a_programming_approach.md) for details
- fully explainable, effective and efficient natural language processing (TBA)
- fully explainable, effective and efficient reinforcement learning (TBA)
- ...

It achieves this "seemingly ambitious" goal via the followings:

- a powerful type system. The type system is powerful enough to express both system-level details and mathematical rigor (inspired mostly by Rust and Lean4).
- ascension mechanism for human in-the-loop. It allows one to effortlessly switch back and forth between human programming(software engineering) and machine automation(AI). Software engineering and AI is seen as special cases of a general process that combines human intelligence, data, and machine computing into making a useful program. For details, see [todo](impress/ascension.md)
- first class support

## Zulip

<https://husky-lang.zulipchat.com>

## Get Started

### Prerequisites

Make sure rust toolchains are installed. To install vscode extension, npm is also required currently.

### Installation

TODO

### Run Examples

TODO

What will probably work is,

```sh
git clone https://github.com/husky-lang-org/husky
cd husky
```

then in one terminal do

```sh
cd gui/gui/husky-tracer-gui-sycamore
trunk serve
```

in another terminal do

```sh
make mnist-compiled
```

then in browser open `localhost:8080`.
## Showcase

## Designs

### Pythonic Syntax

![alt text](snapshots/pythonic-syntax2.png)

### Powerful yet Safe Semantics

All these are possible:

- eager procedural, like C/C++/Rust/python
- eager functional, like OCaml
- lazy functional, like Haskell but advanced to a higher level for the need of machine learning, gui, etc

No interop is needed! (Interop destroys debugging experience, it's good to avoid)

### Everything is Configurable

Husky doesn't make premature assumptions about execution model and memory management.

#### configurable execution

All these are possible:

- interpretation
- compiled to binary
- jit

#### memory management

All these are possible:

- individual alloc/dealloc
- batch alloc/dealloc for (&'eval)
- tracing garbage collector
- automatic refcounting

### Trace-Based Debugging System

Debugging should be as easy as writing the code itself!

Usually devs are designed for procedural languages, because functional ones don't seem to need one. However, the programming problems solved by Husky is intrinsically much harder that a dev is needed even all code is pure functional. For example, in computer vision, blablabla. The major time cost of debugging is to find which line is wrong.

#### generic viewpoint: visualize feature over a subset of datapoints

![alt text](snapshots/trace-based-debugging-system.png)

#### specific viewpoint: visualize feature at a fixed datapoint

![alt text](snapshots/dev-stalk.png)

#### visualization can be customed in type definition

TODO

## Resources

- the Husky Book: TODO
- the Husky Development Book: TODO
- the Husky Machine Learning Book: TODO
- the Husky Frontend Book: TODO
- the Husky Theorem Proving Book: TODO
- the Husky Formal Verification Book: TODO
