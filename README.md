# The Husky Programming Language

(Work In Progress)

Husky is a programming language designed for **next generation computing**, which vastly generalizes nowadays AI and software, which **combines good designs from various current languages and promises killer applications beyond people's imagination**.

## New Sheriff in Town

It serves as an amazing tool with a **uniformed** and much more **ergonomic** developing experience for all the following tasks

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

- a truly powerful type system, inspired mostly by Rust, C++, Haskell, Ocaml and Lean4. The type system is powerful enough to express both system-level details and mathematical rigor.
- ascension mechanism for human in-the-loop, inspired by python deep learning frameworks and scientific computing languages including Mathematica, Matlab, Julia, etc. It allows one to effortlessly switch back and forth between human programming(software engineering) and machine automation(AI). Software engineering and AI is seen as special cases of a general process that combines human intelligence, data, and machine computing into making a useful program. For details, see [todo](impress/ascension.md)
- first class support for debugging, visualization and human computer interaction, inspired by JSX and TSX. This is critical for many things (todo). For details see [todo](impress/notebook.md)
- commitment to a good ecosystem, inspired by Cargo. Corgi/Documentation/Rigor.Security.

## Development History

### Origin

It started as a personal project of Xiyu Zhai, an EECS PhD. In his third year, he was stroke by some interesting ideas in computer vision which could probably be used to craft models that is far more superior than deep learning. But frustrated by the insufficiency of current languages for implementing these ideas, it appeared to him a new language should be created. A year was spent using C++ to write the first versions of Husky, which the design evolved rapidly. But C++ soon turned out to be an insufficient choice even for writing the compiler, as it lacks support for algebraic data type, memory safety and many other basic things, so another year was then spent to create a new version in Rust. In the process, Husky absorbs many good things from Rust, especially the design of `rust-analyzer` and `cargo`.

However, TODO

## Call for Contribution

Husky is not only an open-source project of coding, but is also an open project of research, which means anyone is welcome to join at any time. For example, we currently have an ongoing project on ImageNet which will take 16 months to complete. It's going to involve 100+ people in the final paper. So anyone interested can first join in the project and learn and master coding techniques and machine learning and algorithmic techniques. Afterwards, one can apply the knowledge to other project he or she feel interested in. We'll give everyone full suppport in their own projects using the Husky programming language.

Roughly speaking, contributors can play two roles:

- husky language user. Use the language to do things, like machine learning, reinforcement learning, frontend development, etc. A user in the early stage will help demonstrate the power of the new language, and also help eliminate bugs and improve the design.
- husky language developer. Design and implementation. TODO

Contribution will be faithfully recorded. Those who made innovative contribution will become authors in the corresponding papers.

### Husky Language User

TODO

### Husky Language Developer

TODO

### Zulip

<https://husky-lang.zulipchat.com>

For further details see [CONTRIBUTION.md](CONTRIBUTING.md)

## Getting Started

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