# A tour of crawfish

## Usage

### File structure

All crawfish source code must be written in a UTF-8 encoded file with the `.crw` file extension.

It must contain an entry point called `main()`.

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

## Variables

```
var <variable name>: <type> = <value>;
```

## Values

### Integers

- A 32-bit signed integer
- Pass by value
- Zero value: `0`

```
var x: Int = 5;
```

### Floating-point numbers

- A 64-bit IEEE 754 floating point
- Pass by value
- Zero value: `0`

```
var pi: Float = 3.14;
```

### Booleans

- A `true` or `false` value, 1 byte in size.
- Pass by value
- Zero value: `false`
```
var ready: Bool = false;
var happy: Bool = true;
```

### Characters

- Unicode scalar value i.e. UTF-32
- Exactly one Unicode scalar value. Nothing more, nothing less.
- Enclosed in `'`
- Immutable
- Pass by value
- Zero value:  `'\0'`

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

> **Note**\
> All variables must be declared and initialized. Initializing with the zero value is similar to other languages which implicitly initializes a variable whenever you only declare it.

## Operators

| operator          | category   | description                                                             |
| ----------------- | ---------- | ----------------------------------------------------------------------- |
| `+`               | arithmetic | add                                                                     |
| `-`               | arithmetic | subtract                                                               |
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
| `-=`              | assignment | subtract and assign                                                     |
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

#### Regular usage

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
func even(x: Int) -> bool {
    return if x % 2 == 0 { true } else { false };
}
```

```
var x: Int = 10;
var result: char = if x > 0 { 'y' } else { 'n' };
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

## Functions

```
func <function name>(<parameter type>: <parameter name>) -> <function return type> {
    ...
    return ...;
}
```

## Built-in functions

| Category        | Function   |
| --------------- | ---------- |
| Input/Output    | `println()`|