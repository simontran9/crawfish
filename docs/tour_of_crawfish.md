# A tour of crawfish

> A language which attempts to:
> - Be Fast and simple like Go
> - Be Readable like Python
> - Be high-level like Java
> - Incorporate interesting ideas from Rust

## Usage

### Programming structure

All crawfish source code must be written in a UTF-8 encoded file with the `.crw` file extension.

It must contain an entry point called `main()`

```
func main() {
  // your code
}
```

### Execution

Compile your code with `crawfish build [filename].crw`, then execute it with `./filename`.

For more complicated compilation, Makefiles are the recommended tool.

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

## Comments

```
// single line comment
```

## Values

### Primitive data types

**Integers**
- A 32-bit signed integer
- Pass by value
- Zero value: `0`
```
var x: Int = 5;
```

**Floating-point numbers**
- A 64-bit IEEE 754 floating point.
- Pass by value
- Zero value: `0`
```
const pi: Float = 3.14;
```

**Booleans**
- A `True` or `False` value, 1 byte in size.
- Pass by value
- Zero value: `False`
```
var ready: Bool = False;
var happy: Bool = True;
```

**Characters**
- A 32-bit immutable character enclosed in `'`.
- Pass by value
- Zero value:  `'\x00'`
```
var c: Char = 's';
```

**Strings**
- A heap-allocated immutable text enclosed in `"` for single-lined strings, or `"""` for multi-line strings.
- Pass by reference
- Zero value: `""`
```
var name: String = "Bob";
```

**Fixed-length arrays**
- A heap-allocated fixed-length array of specified type, declared and initialized on the RHS of the equals sign as `[<single populating value>; <length>]`, or `[<element 1>, <element 2>, ..., <element N>]`, and indexing via `<array name>[<index>]`
- Pass by reference
- Zero value: the zero value for the array's type i.e. the zero of the type for a single element
```
var nums: Array[Int] = [0; 5]; // initialize the array to all zero values of length 5
```
```
var fruits: Array[String] = ["apple", "banana", "blueberry"];
const fruit: String = fruits[2] // "bluberry"
```

**Tuples**
- A data structure containing values of specified type
- Pass by value
- Zero value: the zero value depends on the type of each element in the tuple

```
var random: Tuple[String, Float, Int] = ("", 0.0, 0); // initialize to all zero values
```

```
var stuff: Tuple[Int, Char, Float] = (2, 'k', 8.3);
```

> [!NOTE]
> All variables and constants must be declared and initialized.
> Initializing with the zero value is similar to other languages which implicitly initializes
> a variable or constant whenever you only declare it.

### Variables and constants

**Declaration and initialization**
```
var <variable name>: <type> = ...;
```
```
const <constant name>: <type> = ...;
```

**Variable swapping pattern**
```
<first variable>, <second variable> = <second variable>, <first variable>;
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
| `%`               | arithmetic | divison remainder (not the mathematical modulo)                                                                                                    |
| `==`              | comparison | equality                                                                |
| `!=`              | comparison | inequality                                                              |
| `<`               | comparison | less than                                                               |
| `>`               | comparison | greater than                                                            |
| `<=`              | comparison | less or equal than                                                      |
| `>=`              | comparison | greater or equal than                                                   |
| `and`             | logical    | and                                                                     |
| `or`              | logical    | or                                                                      |
| `!`               | logical    | not                                                                     |
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
| `%=`              | assignment | division remainder and assign                                           |
| `&=`              | assignment | bitwise AND and assign                                                  |
| `\|=`             | assignment | bitwise OR and assign                                                   |
| `^=`              | assignment | bitwise XOR and assign                                                  |
| `<<=`             | assignment | bitwise left shift and assign                                           |
| `>>=`             | assignment | bitwise right shift and assign                                          |
| `<start>..<end>`  | range      | exclusive range, where the range is from start, until and excluding end |
| `<start>..=<end>` | range      | inclusive range, where the range is from start, until and including     |

## Control Flow

### Conditional statements

**Regular usage**
```
if <boolean condition> {
    ...
} else if <boolean condition> {
    ...
} else {
    ...
}
```

**Ternary-like usage (i.e. conditional statements as expressions)**
```
func check(x: int) -> String {
    return if x == 2 { "Nice" } else { "No!" };
}
```

```
var x: Int = 10;
var result: string = if x > 0 { "Positive" } else { "Non-positive" };
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

### Structural pattern matching

```
match <variable or constant> {
    ... => ...,
    ... | ... | ... => ...,
    ... if <condition> => ...,
    ... as ... => ...,
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

### Enumeration (algebraic data type)

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

**Plain-old structure definition**
```
struct <struct name> {
    <field name>: <type>,
    <field name>: <type>,
    <field name>: <type>,
}
```

**Associated functions with structures**
```
impl <struct name> {
    // acts like static fields
    const <static name>: <type> = <value>;

    // acts like OOP methods
}
```

**Implementing an interface**
```
struct <struct name> {
    <field name>: <type>,
    <field name>: <type>,
    <field name>: <type>,
}

impl <interface name> for <struct name> {
    ...
}
```

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

| Category        | Function   |
| --------------- | ---------- |
| Input/Output    | `print()`  |
| Input/Output    | `input()`  |
| Type Conversion | `int()`    |
| Type Conversion | `float()`  |
| Type Conversion | `string()` |
| File Handling   | `open()`   |
| File Handling   | `close()`  |
| Math            | `min()`    |
| Math            | `max()`    |
| Math            | `abs()`    |
| Math            | `pow()`    |
| Math            | `sqrt()`   |
| Math            | `ceil()`   |
| Math            | `floor()`  |
| Math            | `sin()`    |
| Math            | `cos()`    |
| Math            | `log()`    |
| Math            | `ln()`     |
| Math            | `pow_e()`  |

## Standard library

### Containers

Abstract data types are implemented with the most common operations — access, insertion, deletion, and sometimes, updates.

- `ArrayList`: a list backed by a dynamic array
- `LinkedList`: a list backed by a linked list
- `ArrayStack`: a stack backed by a dynamic array
- `ArrayQueue`: a queue backed by a circular dynamic array
- `HeapPriorityQueue`: a priority queue backed by a binary heap
- `HashMap`: a map backed by a hash table
- `HashSet`: a set backed by a hash table
- `RBTreeSorderedMap`: a ordered map backed by a Red-Black Tree
- `RBTreeOrderedSet`: an ordered set backed by a Red-Black Tree

### Algorithms

- `sort`: driftsort stable sorting algorithm
- `binary_search`: binary search algorithm

### Threads

Crawfish supports operating systems threads, called `os_thread`, and green threads, called `green_thread`.

### Synchronization primitives

**`MutexLock`**

**`CondVar`**

**`Semaphore`**

**`RwLock`**

**`Atomic`**

**`Channel`**

### Inter-process communication mechanisms

