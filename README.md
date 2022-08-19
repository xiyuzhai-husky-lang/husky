# Work in Progress

The Husky Programming Language

Make programming great again!

:warning: this repo is still under heavy construction: documentation is still serious lacking, number of todos are above 2000, and rustc warnings are everywhere!

## Introduction

Husky is **a programming language designed for many difficult tasks which haven't yet been seen as programmable at all**. For example, we aims towards using Husky for writing a strongly explainable and efficient classifier for image recognition tasks at any scale. Husky can actually do well in traditional programming domains, but the current focus is on exploring the boundary of programming.

The major advancement is the language frontend. In a long time, Husky will rely on Rust/Zig/C/Cpp for compilation so that we don't need to spend too much time in dealing with IRGen.

For a quick review of core ideas in computer vision, see [vision-a programming approach](sparks/impress/vision_a_programming_approach.md).

## Get Started

### Prerequisites

Make sure rust toolchains are installed. To install vscode extension, npm is also required currently.

### Installation

TODO

### Run Examples

TODO

## Motivation

### A Bright Future of AI

The core belief is the existence of a strongly explainable program (as explainable as the software we wrote) for a range of AI problems, including computer vision, natural language processing. And Husky is designed to be the language to write that program.

Now "explainability" is important for two reasons:

- allows people to collaborate
- allows task specific optimization

In fact, a truly explainable AI can be compared with software developed by humans:

datasets -- tests
training -- development
inference -- runtime

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

Usually debuggers are designed for procedural languages, because functional ones don't seem to need one. However, the programming problems solved by Husky is intrinsically much harder that a debugger is needed even all code is pure functional. For example, in computer vision, blablabla. The major time cost of debugging is to find which line is wrong.

#### generic viewpoint: visualize feature over a subset of datapoints

![alt text](snapshots/trace-based-debugging-system.png)

#### specific viewpoint: visualize feature at a fixed datapoint

![alt text](snapshots/debugger-stalk.png)

#### visualization can be customed in type definition

TODO

## About Development

This project was created by and is currently maintained solo by Xiyu Zhai, a Phd in MIT EECS, whose primary background is actually in pure math and has just programmed intensively for the last two years. So please help, smart people!

It was created as a language to implement certain ideas for efficient image classification, which is hard to do in existing languages. Originally it was written in C++, and had gone through many versions that were influenced heavily by C++. However, the lacking of clean pattern matching (like Rust enum) and memory safety and many other things matching make development hard. Then, a Rust version was created from scratch and despite of the learning curve, the development is quite smooth and the design of language is becoming very similar to Rust. Still, value binding is syntactically similar to that of C++ because of succinctness, whereas semantically safety is still guaranteed, just like in Rust.

## Khala Links

### Talks

#### language talks

"Outperforming Imperative with Pure Functional Languages" by Richard Feldman <https://www.youtube.com/watch?v=vzfy4EKwG_Y/>

### Repositories

#### language repos

Interesting new languages

Lean 4 <https://github.com/leanprover/lean4/>

Zig <https://github.com/ziglang/zig.git/>

Roc <https://github.com/roc-lang/roc/>

### ml frameworks

Google Jax <https://github.com/google/jax/>

Torch Quantum <https://github.com/mit-han-lab/torchquantum/>

## Youtube Channels

### programming

Rust <https://www.youtube.com/c/RustVideos/>

Niko Matsakis <https://www.youtube.com/user/nikomatsakis/>

Healthy Software Developer <https://www.youtube.com/c/JaymeEdwardsMedia/>

Jon Gjengset <https://www.youtube.com/c/JonGjengset/>

ThePrimeagen <https://www.youtube.com/c/ThePrimeagen/>

### huskies

Gone to the Snow Dogs <https://www.youtube.com/gttsd/>

Maya Husky <https://www.youtube.com/c/MayaHusky/>

K'eyush The Stunt Dog <https://www.youtube.com/c/KeyushTheStuntDog/>

Gohan The Husky <https://www.youtube.com/c/GohanTheHusky/>

Skaya Siberian <https://www.youtube.com/c/SkayaSiberian/>

Sixty Formula <https://www.youtube.com/c/SixtyFormula/>
