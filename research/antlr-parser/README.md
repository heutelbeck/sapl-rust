# ANTLR PARSER

This crate contains the antlr grammar definition and the auto generated files to parse sapl policies.

## How to generate the parser files

Java must be installed. Then you can run the following cmd:

```
java -jar tool/antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust grammar/SAPL.g4 -Xexact-output-dir -o src/gen -visitor
```
