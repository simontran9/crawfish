# Design

## Compiler

### High-level overview

1. Lexical Analysis
   - Source Code → Tokens
   - Converts raw source code into a sequence of tokens
   - No understanding of meaning, just recognizing keywords, identifiers, literals, etc.

2. Parsing
   - Tokens → AST
   - Constructs an Abstract Syntax Tree (AST)
   - Still mostly syntactic; ensures code follows grammatical rules

3. Semantic Analysis
   - AST → AST with Annotations
   - Checks type correctness, scoping rules, function arity, control flow validity, etc.
   - Resolves names (e.g., variable and function lookups)
   - Annotates AST nodes with semantic information (types, symbol table entries)
   - Detects semantic errors before code generation

4. Intermediate Representation (IR) Generation
   - AST → IR (Basic Blocks, Control Flow Graph)
   - Transforms AST into an IR, like SSA or three-address code
   - More structured and optimized for later passes

5. Code Generation
   - IR → Machine Code
   - Translates IR to assembly or machine code
   - Includes optimizations, register allocation, instruction selection

### Frontend

The scanner is handwritten scanner, largely based on Go's internal scanner.

The parser is a recursive descent parser, while making use of Pratt parsing.

The AST is designed around the idea of struct of arrays for cache friendliness.

https://www.reddit.com/r/ProgrammingLanguages/comments/1hwfzj9/comment/m69kvh7/

https://ziglang.org/download/0.8.0/release-notes.html#Reworked-Memory-Layout

### Backend

## Syntax design

### Grammar

> **Theorem**\
> Crawfish's grammar is context-sensitive, and not, context-free.

**Proof**\
Assume for a contradiction, that the Context-free language.

https://soc.me/languages/stop-using-angle-brackets-for-generics.html
