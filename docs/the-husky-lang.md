# Introduction

Husky is a system-level multiparadigm programming language that is safe and concise, with builtin debugging and visualization. Its core feature is task-oriented functional programming.

## History

Originally developed for implement ideas for interpretable and efficient machine learning in computer vision, but turns to be able to do much more.

## Task

`Task` is the most important concept in Husky that other languages don't have.

Each package is designated by one task, as user shall define in the main file.

For example,

```python
task:
    domains::ml::cv::classify_mnist()
```

This specifies that the package is intended to describe a model and its training methods to class the Mnist dataset.

For the task in example, it will introduce implicitly a placeholder `input`, representing input of the Mnist dataset together with Symbols `Input`, `Output` representing types of input and output.

One can define a 'function' that depends only on `input` by

```python
proc trivial_feature -> r32:
    return input[0]
```

It reads as a 'function' of name `trivial_feature` and output type `r32` and a definition block of paradigm `procedural`. Note that there is no bracket after the function name indicating that this function is binded to `input` which is brought by the task.

Functions written like this are called `feature`.

To use it in other places (if semantically legal), one just use its function name, without any parentheses.

```python
def trivial_feature2 -> r32:
    trivial_feature
```

Here `def` is another paradigm keyword which means `lazy functional`. Under `lazy functional` paradigm, a return statement is just an expression that take one line group.

Now we can define functions that takes in one feature and create one:

```python
def trivial_feature_gen(f: r32) -> r32:
    f
```

Note here we simply write the type of `f` as `r32`, and it's implicitly interpreted as `Input -> r32`.
This is what we call task-specific ascension.

In fact, we can also ascend `r32` to `TrainingData Input -> r32` for training.

So there are multiple contexts for a given task, and each context has its way of type ascension.

For example, a linear regression can be written as

```python
def linear_regression<n: usize>(f: [f32;n]) -> f32:
    ...
```

The use of ascension makes type specification very concise.

In general, an ascension is defined as a map from types to types, such that operations are reserved.
In the case above, there are addition, subtraction, etc for `f32`, and we have induced operations for `Input->f32` by pointwise calculation. Things generalize to function and methods. For example, we can compose `A->B` and `B->C`, we can also compose `Input->A->B` and `Input->B->C` by doing pointwise composition. This reservation of operation allows us to write code while simply thinking features as constants depending on `input`.

For GUI development, the user input is seen as a stream, say `Stream<UserEvent>`, then type ascension of a type `A` is then
`Stream<UserEvent> -> Time -> A`
