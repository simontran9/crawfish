# A tour of crawfish

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

#### Integers

- A 32-bit signed integer
- Pass by value
- Zero value: `0`
```
var x: Int = 5;
```

#### Floating-point numbers

- A 64-bit IEEE 754 floating point.
- Pass by value
- Zero value: `0`
```
var pi: Float = 3.14;
```

#### Booleans

- A `True` or `False` value, 1 byte in size.
- Pass by value
- Zero value: `False`
```
var ready: Bool = False;
var happy: Bool = True;
```

#### Characters

- Unicode scalar value i.e. UTF-32
- Exactly one Unicode scalar value. Nothing more, nothing less.
- Enclosed in `'`
- Immutable
- Pass by value
- Zero value:  `'\0'`, `'\x00'`, `'\u{0}'`

```
var c: Char = 's';
```

| Escape Sequence | Description        |
| --------------- | ------------------ |
| `\'`            | Single quote       |
| `\"`            | Double quote       |
| `\n`            | Newline (U+000A)   |
| `\r`            | Carriage return    |
| `\t`            | Tab (U+0009)       |
| `\\`            | Backslash          |
| `\0`            | Null byte (U+0000) |
| `\xNN`          | Hexadecimal escape (2 hex digits, 0x00–0x7F only) |
| `\u{XXXXXX}`    | Unicode escape (1–6 hex digits, U+0000–U+10FFFF) |

#### Strings

- A heap-allocated immutable text enclosed in `"` for single-lined strings, or `"""` for multi-line strings.
- Pass by reference
- UTF-8 encoded
- Zero value: `""`
```
var name: String = "Bob";
```

#### Fixed-capacity arrays

- A heap-allocated fixed-capacity array of specified type, declared and initialized on the RHS of the equals sign as `[<single populating value>; <length>]`, or `[<element 1>, <element 2>, ..., <element N>]`, and indexed via `<array name>[<index>]`
- Pass by reference
- Zero value: the zero value for the array's type i.e. the zero of the type for a single element
```
var nums: Array[Int] = [0; 5]; // initialize the array to all zero values of length 5
```
```
var fruits: Array[String] = ["apple", "banana", "blueberry"];
var fruit: String = fruits[2] // "bluberry"
```

#### Tuples

- A data structure containing values of specified type
- Pass by value
- Zero value: the zero value depends on the type of each element in the tuple

```
// initialize to all zero values
var random: Tuple[String, Float, Int] = ("", 0.0, 0);

// tuple initialized with different elements
var stuff: Tuple[Int, Char, Float] = (2, 'k', 8.3);

// destructering
var (x, y, z) = stuff;

// accessing through indexing
var first_val = stuff[0];
var second_val = stuff[1];
var third_val = stuff[2];
```

> [!NOTE]
> All variables and varants must be declared and initialized. Initializing with the zero value is similar to other languages which implicitly initializes a variable whenever you only declare it.

### Variables

#### Declaration and initialization

```
var <variable name>: <type> = ...;
```

#### Variable swapping pattern

```
<first variable>, <second variable> = <second variable>, <first variable>;
```

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

#### Regular usage**
```
if <boolean condition> {
    ...
} else if <boolean condition> {
    ...
} else {
    ...
}
```

#### Ternary-like usage (i.e. conditional statements as expressions)
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
match <variable or varant> {
    ... => ...,
    ... | ... | ... => ...,
    ... if <condition> => ...,
    ... => ...,
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

#### Plain-old structure definition

```
struct <struct name> {
    <field name>: <type>,
    <field name>: <type>,
    <field name>: <type>,
}
```

#### Associated functions with structures

```
<struct name> {
    // acts like static fields
    var <static name>: <type> = <value>;

    // acts like a method an on object accessed via <object>.<method name>(<arg>)
    func <method name>(this, <arg>) -> <return type> {
        ...
    }

    // acts like a static method an on struct accessed via <struct name>::<method name>(<arg>)
    func <method name>(<arg>) -> <return type> {
        ...
    }
}
```

#### Implementing an interface

```
struct <struct name> implements <interface name> {
    <field name>: <type>,
    <field name>: <type>,
    <field name>: <type>,
}
```

## Generics

```
func <function name>[T](<parameter name>: T) -> T {
    <var or var> <container name>: T = ...;
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

## Import

TO DO

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

Abstract data types are supported with the most common operations: access, insertion, deletion, and sometimes, updates.

- `ArrayList`: a list backed by a dynamic array
- `SinglyLinkedList`: a list backed by a singly linked list
- `DoublyLinkedList`: a list backed by a doubly linked list
- `ArrayStack`: a stack backed by a dynamic array
- `LinkedStack`: a stack backed by a singly linked list
- `ArrayQueue`: a queue backed by a circular dynamic array
- `LinkedQueue`: a queue backed by a singly linked list
- `MinHeapPriorityQueue`: a priority queue backed by a min binary heap
- `MaxHeapPriorityQueue`: a priority queue backed by a max binary heap
- `HashMap`: a map backed by a hash table
- `HashSet`: a set backed by a hash table
- `RBTreeOrderedMap`: a ordered map backed by a Red-Black Tree
- `RBTreeOrderedSet`: an ordered set backed by a Red-Black Tree

### Algorithms

- `sort`: driftsort stable sorting algorithm
- `binary_search`: binary search algorithm
