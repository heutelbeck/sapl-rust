# SAPL Policy parsing with pest

## [pest. The Elegant Parser](https://pest.rs/)
pest is a general purpose parser written in Rust. It uses [parsing expression grammars (or PEG)](https://en.wikipedia.org/wiki/Parsing_expression_grammar) as input, which are similar in spirit to regular expressions, but which offer the enhanced expressivity needed to parse complex languages.

## How it works
The sapl grammar definition in pest is located in the grammar folder.
The grammar is read into src/lib.rs and the code required for parsing is generated at compile time. The code in src/lib.rs reads the data-stream from the parser and generates the wanted data objects / structer.
