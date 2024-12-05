# visored

A tool for verifying natural language reasoning.

Using NLP techniques and rules, it translates detailed natural language proofs into formal proofs.

It is designed with the following goals:

- data efficiency. No need for large amount of formal language data.
- computation efficiency. Implemented in Rust. Optimized for speed. More than thousands of times faster than existing tools.
- configurability. Behaviors can be specified by various configuration files. It's easy to configure it to output to different formal languages like Lean, Rcoq, Isabelle, Metamath, etc.
- extensibility. It's able to handle quite arbitrary natural language inputs by combining NLP techniques and rules. Husky will be used to scale up the system to its true potential.

The pipeline of visored is as follows:

1. Rewrite stage. Rewrite the input into a controlled natural language, using both NLP and rules.
2. Latex stage. Use latex parser to parse the input into a tree structure.
3. Syntax stage. Perform constituency parsing and formula parsing. Resolve symbols.
4. Semantic stage. Type inference, checking and function dispatch.
5. Mir stage. Reduce the syntax tree into a middle intermediate representation (MIR) for convenience.
6. Tactics stage. Fill in the missing pieces using tactics.
7. Output stage. Translate the MIR into a target formal language like Lean, Rcoq, Isabelle, Metamath, etc for soundness.

It's different from previous projects like Naproche in that

- The CNL(Controlled Natural Language) in visored is not determined by pure rules, but also learned functions over natural language data. Human users don't need to learn CNL to use visored. And LLMs for reasoning don't need extra data to be verified.
- It's not meant to be a self-consistent system. Signatures are specified by configuration files. Soundness relies on external tools.
- It's implemented in a system-level language instead of Haskell, optimized for speed and large-scale engineering.

It's significance if design goals are achieved:

- For interactive theorem proving, it aims to be a fast local analyzer for natural language proofs. Mathematicians or students can have the majority of their proofs verified in real time, without any needs to learn tools like Lean, Coq, etc.
- For automatic theorem proving, it provides a fast and reliable reward function for RL algorithms to train on.
