# Future language additions

## Addition 1

- make runner mode
- add bench step to ci
- add pre-packaged binaries and add step to ci

## Addition 2

- Characters
    - allow hex and unicode escape sequence in char
    add this to table
    | `\xNN`          | Hexadecimal escape (2 hex digits, 0x00–0x7F only) |
    | `\u{XXXXXX}`    | Unicode escape (1–6 hex digits, U+0000–U+10FFFF) |
    and mention new zero values
    `'\x00'`, or `'\u{0}'`
- Strings
    - Single-line strings enclosed by `"`
        - A heap-allocated immutable text enclosed in `"` for single-lined strings.
        - Pass by reference
        - UTF-8 encoded
        - Zero value: `""`
        ```
        var name: String = "Bob";
        ```
    - Multi-line strings enclosed by `""""`
    - String interpolation feature via `{}`
- Built-in functions (no import required)
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

## Addition 3

- Imports
- Fixed-capacity arrays
    - A heap-allocated fixed-capacity array of specified type, declared and initialized on the RHS of the equals sign as `[<single populating value>; <length>]`, or `[<element 1>, <element 2>, ..., <element N>]`, and indexed via `<array name>[<index>]`
    - Pass by reference
    - Zero value: the zero value for the array's type i.e. the zero of the type for a single element

    ```
    var nums: Array[Int] = [0; 5]; // initialize the array to all zero values of length 5
    ```

    ```
    var fruits: Array[String] = ["apple", "banana", "blueberry"];
    var fruit: String = fruits[2] // "blueberry"
    ```
- Tuples
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
- Variable swapping pattern
    ```
    <first variable>, <second variable> = <second variable>, <first variable>;
    ```

## Addition 4

- Structural pattern matching
    ```
    match <variable or varant> {
        ... => ...,
        ... | ... | ... => ...,
        ... if <condition> => ...,
        ... => ...,
        _ => ...,
    }
    ```
- Scope-based defer
    ```
    func <name>() {
        ...
        defer ...;
        ...
    }
    ```
- Tagged unions
    ```
    enum <enum name> {
        <variant 1>,
        <variant 2>,
        <variant 3>,
        <...>,
        <variant N>,
    }
    ```

## Addition 5

- Structures
    - Plain-old structures
        ```
        struct <struct name> {
            <field name>: <type>,
            <field name>: <type>,
            <field name>: <type>,
        }
        ```
    - Structures with methods
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
- Interfaces
    ```
    interface <interface name> {
        func <function name>(<parameter name>: <parameter type>) -> <return type>;
    }
    ```
    ```
    struct <struct name> implements <interface name> {
        <field name>: <type>,
        <field name>: <type>,
        <field name>: <type>,
    }
    ```

## Addition 6

- Generics
    ```
    func <function name>[T](<parameter name>: T) -> T {
        var <container name>: T = ...;
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
- Standard library
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
    - `sort`: driftsort stable sorting algorithm
    - `binary_search`: binary search algorithm
