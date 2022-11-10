# VISION: A PROGRAMMING APPROACH

## Existing Approaches to Vision

### Traditional feature-based approaches

Handcrafted feature descriptors such as SIFT and SURF are fed to traditional
machine learning classification algorithms such as Support Vector Machines and k-Nearest Neighbours to solve the aforementioned CV problems.

Biggest problem: doesn't work well for large datasets.

### End-to-end approaches

Deep learning.

Biggest problem: doesn't explain well, totally not scientific.

## A New Programming-Based Approach

Here we present a new programming-based approach, which can generate models that are truly interpretable and efficient.

### Mathematical Insights

This section is skippable, but for those with enough mathematical maturity, reading through section will greatly help understanding of the big picture.

#### Invariants

In mathematics, we use category theory to formalize the study of abstract objects.
In a category, two objects are isomorphic iff there is an isomorphism between them, where an isomorphism from $A$ to $B$ is a morphism that has an double inverse.
So there arises naturally the mathematical problem of telling whether two given objects are isomorphic, such as

- isomorphism of two graphs
- isomorphism of topological spaces
- isomorphism of groups
- isomorphism of schemes

When things can be represented in finite terms, this problem can also be seen as a computational one.
However, from definition one can only argue that the problem is $NP$ for many cases. For example, isomorphism of graphs are actually NP-hard.
But the good new is, the problem is not always NP-hard. Mathematicians have used `invariants` to simplify the isomorphism problem.

An invariant is roughly a mathematically defined mapping from a category to a set, such that isomorphic objects are mapped to the same value.
If we're lucky that this invariant completely determines isomorphic classes, then basically the mathematical problem is solved.
If furthermore, the invariant is computationally easy to compute (at least in polynomial time), then the computational problem is also solved.

Examples of smart invariants:

- genus of algebraic curve
- euler characteristics

How is this related to machine learning?

An ideal pattern recognition can be seen as such: given a set of labeled object, for each new object, find the object isomorphic to it, and return the label.

Traditional computer vision uses many template matching, which is inherently inefficient.
If we can find smart features (corresponding to smart invariants in an ideal world), then things can be much easier.

It's important to model things appropriately. The concept of `shape` is intrinsically a computational one,
because it's formed from the interaction of brains finite capacity with outer physical world.
It's somewhere between `Riemannian` and `topological`.
It's important to design features that characterizes what between `Riemannian` and `topological`, not too coarse, nor too fine-grained.

TODO

#### Global Machine Learning

In mathematics, geometry basically is reinterpreted as the study of global structures that comes from combinations of local structures.

Examples:

- differential manifold is defined by a compatible collection of local charts of eulidean open sets
- schemes are compatible varieties pieced together

In machine learning, we try to see a real world dataset as a collection of small machine learning problems

TODO

#### Singularity

support vector machine, points on the boundary matters, it's like divisor

### The Model for Mnist

Let's first see what the model is like in the case of Mnist.

We define the following features:

- `raw_input`: this is the raw input of mnist dataset, $28\times 28$ gray scale image;

- `binary_image`: a binary image obtained from thresholding `raw_input`.

- `connected_components`: the set of connected components of `binary_image`.

- `major_connected_component`: a single connected region obtained from connected_components so that:

  - equals `connected_components[0]` when there is only one connected components

  - when there are two very close connected components, say, $A$ and $B$, then `major_connected_component` contains points that satisfies one of

    - is a point in $A$
    - is a point in $B$
    - is a point in a path from $A$ to $B$ of length no more than $dist(A, B) + \epsilon$, where $dist(A, B)$ is the canonical distance between $A$ and $B$ and $\epsilon$ is a very small positive number.

  - otherwise, equals the largest one

- `major_raw_contours`: the set of contours of `major_connected_component` represented by `Vec<Vec<Point2d>>`. The convention is that `major_raw_contours[0]` is always the counterclosewise (also the outer) one.

![alt text](../snapshots/generic-major-raw-contours.png)

- `major_line_segment_sketch`: line segment sketch of `major_raw_contours[0]`. This sketching is roughly speaking, very similar to that in painting. This is key to our interpretable and efficient processing of shapes. It's quite natural, as this technique has been developed in painting for thousands of years.

![alt text](../snapshots/generic-major-line-segment-sketch.png)

- `major_concave_components`

![alt text](../snapshots/generic-major-concave-components.png)
