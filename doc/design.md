# Design

## Compiler design

I wrote a handwritten scanner and a recursive descent parser.

The AST was a [struct of arrays](https://en.wikipedia.org/wiki/Entity_component_system) for cache friendliness

https://www.youtube.com/watch?v=KOZcJwGdQok

https://github.com/ziglang/zig/blob/master/lib/std/zig/Ast.zig

## Grammar

Context-sensitive. 

Assume for a contradiction, that the Context-free language.
