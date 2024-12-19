# A tour of crawfish

## Usage

### Programming structure

All crawfish source code must be written in a UTF-8 encoded file with the `.crw` file extension.

It must contain an entry point called `main()`

```
func main() -> void {
  // your code
}
```

### Execution

Compile your code with `crawfish build [filename].crw`, then execute it with `./filename`

For more complicated compilation, Makefiles are the recommended tool

## Style guide

| data type   | naming convention |
| ----------- | ----------------- |
| file        | `snake_case`      |
| variable    | `snake_case`      |
| function    | `snake_case`      |
| method      | `snake_case`      |
| enumeration | `PascalCase`      |
| structure   | `PascalCase`      |

> [!NOTE]
> indent with four spaces

> [!WARNING]
> identifier's first character can only begin with a letter, but subsequent characters may be a letter, digit, or an underscore

> [!TIP]
> use the upcoming `crawfish fmt` for automatic formatting

## Comments

```
// single line comment
```

```
/* multiline 
comment */
```

## Values

### Primitive data types

| data type                   | category | pass type         | description                                                                      |
| --------------------------- | -------- | ----------------- | -------------------------------------------------------------------------------- |
| `int`                       | atomic   | pass by value     | a 32-bit signed integer                                                          |
| `float`                     | atomic   | pass by value     | a 64-bit IEEE 754 floating point number                                          |
| `bool`                      | atomic   | pass by value     | a 1-bit `True` or `False` boolean value                                          |
| `char`                      | atomic   | pass by value     | a 32-bit character enclosed in `'`                                               |
| `string`                    | compound | pass by reference | a null-terminated `char` array denoted using `"` or `"""` for multi-line strings |
| `(<type 1>, <type 2>)` | compound | pass by value     | a tuple containing two values of specified type enclosed in `()`                 |
| `[<type>]`   | compound | pass by reference | an array of specified type with length, enclosed in `[]`                          |

**optional**  
`annotate <type>?` if the value may be `None`

### Variables and constants

```
var <variable name>: <type> = ...;
```

```
const <constant name>: <type> = ...;
```

### Supported escape sequences

| Escape Sequence | Description         |
|-----------------|---------------------|
| `\n`            | newline             |
| `\r`            | carriage return     |
| `\t`            | tab                 |
| `\b`            | backspace           |
| `\f`            | form feed           |
| `\\`            | backslash           |
| `\'`            | single quote        |
| `\"`            | double quote        |

## Operators

| operator          | category   | description                                                             |
| ----------------- | ---------- | ----------------------------------------------------------------------- | 
| `+`               | arithmetic | add                                                                     |
| `-`               | arithmetic | substract                                                               |
| `*`               | arithmetic | multiply                                                                |
| `/`               | arithmetic | divide                                                                  |
| `%`               | arithmetic | modulo divide                                                           |
| `==`              | comparison | equality                                                                |
| `!=`              | comparison | inequality                                                              |
| `<`               | comparison | less than                                                               |
| `>`               | comparison | greater than                                                            |
| `<=`              | comparison | less or equal than                                                      |
| `>=`              | comparison | greater or equal than                                                   |
| `and`             | logical    | and                                                                     |
| `or`              | logical    | or                                                                      |
| `not`             | logical    | not                                                                     |
| `&`               | bitwise    | AND                                                                     |
| `\|`              | bitwise    | OR                                                                      |
| `^`               | bitwise    | XOR                                                                     |
| `~`               | bitwise    | NOT                                                                     |
| `<<`              | bitwise    | left shift                                                              |
| `>>`              | bitwise    | right shift                                                             |
| `=`               | assignment | assign                                                                  |
| `+=`              | assignment | add and assign                                                          |
| `-=`              | assignment | substract and assign                                                    |
| `*=`              | assignment | multiply and assign                                                     |
| `/=`              | assignment | divide and assign                                                       |
| `%=`              | assignment | modulo divide and assign                                                |
| `&=`              | assignment | bitwise AND and assign                                                  |
| `\|=`             | assignment | bitwise OR and assign                                                   |
| `^=`              | assignment | bitwise XOR and assign                                                  |
| `<<=`             | assignment | bitwise left shift and assign                                           |
| `>>=`             | assignment | bitwise right shift and assign                                          |
| `<start>..<end>`  | range      | exclusive range, where the range is from start, until and excluding end |
| `<start>..=<end>` | range      | exclusive range, where the range is from start, until and including     |

## Control Flow

### Conditional statements

```
if <boolean condition> {
    ...
} else if <boolean condition> {
    ...
} else {
    ...
}
```

### Loops

```
while <condition> {
    ...
}
```

```
for <item> in <iterable collection> {
    ...
}
```

```
for <item> in <range> {
    ...
}
```

> [!NOTE]
> to step by a certain amount, use the `step` keyword and a positive or negative integer for the amount to step by

### Match

```
match <variable or constant> {
    ... => ...,
    ... | ... | ... => ...,
    <start>..<end> => ...,
    <start>..=<end> => ...,
    _ => ...,
}
```

### Defer

```
func <name>() {
    ...
    defer ...;
    ...
}
```

## Functions and interfaces

### Function

```
func <function name>(<parameter type>: <parameter name>) -> <function return type> {
    ...
    return ...;
}
```

### Interface

```
interface <interface name> {
    func <function name>(<parameter name>: <parameter type>) -> <return type>;
}
```

## User-defined types

### Enumeration

```
enum <enum name> {
    <variant 1>,
    <variant 2>,
    <variant 3>,
    <...>,
    <variant N>,
}
```

### Structure

```
struct <struct name> {
    field <field name>: <type>;
    <var or const> <container name>: <type>;

    func new(<parameter name>: <parameter type>) -> <struct name> {
        this.<field name> = <parameter name>;
        ...
    }

    func <method name>(<parameter name>: <parameter type>) -> <return type> {
        ...
    }
}
```

> [!NOTE]
> if you want a struct to follow an interface, the struct declaration should be `struct <struct name> implements <interface name>`

## Generics

```
func <function name>[T](<parameter name>: T) -> T {
    <var or const> <container name>: T = ...;
}
```

```
struct <struct name>[T] {
    ...
}
```

```
interface <interface name>[T] {
    ...
}
```

## Import and package

## Built-in functions

| Category              | Function            |
|-----------------------|---------------------|
| Input/Output          | `print()`           |
| Input/Output          | `println()`         |
| Input/Output          | `input()`           |
| Type Conversion       | `int()`             |
| Type Conversion       | `float()`           |
| Type Conversion       | `string()`          |
| File Handling         | `open()`            |
| File Handling         | `close()`           |
| File Handling         | `read()`            |
| File Handling         | `write()`           |

## Standard library

### Collections

Abstract data types are implemented with the most common operations — access, insertion, deletion, and sometimes, updates.

- `List`: a list backed by a dynamic array
- `Stack`: a stack backed by a dynamic array
- `Queue`: a queue backed by a circular dynamic array
- `PriorityQueue`: a priority queue backed by a binary heap
- `Map`: a map backed by a hash table
- `Set`: a set backed by a hash table
- `SortedMap`: a sorted map backed by a Red-Black Tree
- `SortedSet`: an sorted set backed by a Red-Black Tree

### Algorithms

- `sort`
- `binary_search`

### Concurrency primatives

- `os_thread`
- `green_thread`
- `Mutex`
- `Cond`
- `atomic`
- `Semaphore`
- `channel`

