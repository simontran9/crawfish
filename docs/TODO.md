# TO DO

https://hy-compilers.github.io/spring-2025/
- https://www.reddit.com/r/Compilers/comments/109uwhf/which_book_to_choose/?rdt=48184
- implement regular string
- implement multi string
- implement char
- re-architect https://github.com/rust-analyzer/ungrammar/blob/42810d770e4cddec2a5fff658489fa72f3b28a7c/src/lexer.rs
- implement position Scanner field for compiler errors https://github.com/zkat/miette
```
struct Location {
    start: usize;
    end: usize;
};
```
- profile scanner by checking for hotspots using perf and reviewing flamegraphs
- write parser https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
- write AST as SOA https://www.youtube.com/watch?v=KOZcJwGdQok, https://github.com/ziglang/zig/blob/master/lib/std/zig/Ast.zig
