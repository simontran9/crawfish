# Progress

- make perfect minimal hashmap for keyword lookups
- implement regular string
- implement multi string
- implement char
- implement position Scanner field for compiler errors like this https://github.com/brendanzab/codespan  or https://github.com/zesterer/ariadne?tab=readme-ov-file
```
struct Pos {
    size_t start;
    size_t end;
};
```
- profile scanner by checking for hotspots using perf and reviewing flamegraphs
- write Makefile 
- properly setup GH Actions
- write parser
- write AST as SOA
- add fuzz testing and prop testing