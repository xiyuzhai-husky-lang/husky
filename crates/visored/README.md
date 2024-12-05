# visored

A tool for verifying natural language reasoning.

Using NLP techniques and rules, visored translates natural language proofs into formal proofs.

## Goals

- **Data Efficiency**: Operates without requiring large formal language datasets
- **Performance**: Implemented in Rust and optimized for speed, achieving performance thousands of times faster than existing tools
- **Configurability**: Supports customization through configuration files, easily adapting to output different formal languages (Lean, Coq, Isabelle, Metamath, etc.)
- **Extensibility**: Handles arbitrary natural language inputs by combining NLP techniques with rules. Integration with Husky enables scaling the system to its full potential

## Pipeline

1. **Rewrite Stage**: Convert input into controlled natural language using NLP and rules
2. **LaTeX Stage**: Parse input into a tree structure using LaTeX parser
3. **Syntax Stage**: Perform constituency parsing and formula parsing, resolving symbols
4. **Semantic Stage**: Execute type inference, checking, and function dispatch
5. **MIR Stage**: Reduce syntax tree into a Middle Intermediate Representation (MIR)
6. **Tactics Stage**: Fill in missing pieces using tactics
7. **Output Stage**: Translate MIR into target formal languages (Lean, Coq, Isabelle, Metamath, etc.) for soundness verification

## Comparison with Naproche

Key differentiators:

- **Advanced CNL**: The Controlled Natural Language in visored combines rules with learned functions from natural language data. Users and LLMs don't need special training to use the system
- **External Verification**: Rather than maintaining internal consistency, visored relies on external tools for soundness verification, with signatures specified via configuration
- **Performance-Focused**: Built in Rust for superior speed and scalability, compared to Haskell-based alternatives

## Potential Impact

### Interactive Theorem Proving
- Provides real-time verification of natural language proofs
- Eliminates the need for mathematicians and students to learn specialized formal languages

### Automated Theorem Proving
- Serves as a fast, reliable reward function for reinforcement learning algorithms
