# Quenya

Natural Language Processing and Generation in Rust.

Working title "Quenya", meaning "language" in Tolkien's constructed language for the ancient Elves.

## Introduction

Quenya is a library/framework with utilities for analyzing and manipulating (written) language utterances.

It is intended:
- as a language-agnostic library through cross-compilation into WebAssembly and bindings to other languages.
- to be generic enough to (eventually) support any language that can be represented by symbols.
- to be fast by using Rust and parallel processing

Most NLP libraries are written in Python and rely on data sets that are very specific.
These libraries often provide useful tools for language analysis and Named Entity Recognition, but not for language manipulation.
What's missing from libraries, such as SpaCy, is determining what case/gender/etc a word is.
Or to generate a word from a lemma and a set of grammatical specifiers.

That's what Quenya aims to provide.

## Planned Features

### Language utilities

- Lemmatization
- Lexicon lookup
- Verb Conjugation and parsing
- Grammatical pattern recognition and realization

### Annotation and Language Information

- Annotate a word's grammatical properties
- Generate grammatical tables
- Annotate grammatical patterns

## Usage

TBD.

## Build

Run `cargo build` to build the project.

### TypeScript module

To build, run:

```
wasm-pack build --out-name quenya
```

This creates a JS/TS module in wasm/pkg.

To use the generated module in a Node project locally, first run `yarn link` inside the `wasm/pkg` directory.
Then run `yarn link quenya` in the project that you want to use the locally linked module in.
