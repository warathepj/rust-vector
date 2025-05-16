# Vector Demo in Rust

This project demonstrates the usage of Vectors in Rust, showing common operations and best practices for working with this fundamental data structure.

## Purpose

The purpose of this project is to provide a practical demonstration of Rust's `Vec<T>` type (vector), which is a dynamic array implementation that allows storing multiple values of the same type in contiguous memory. This demo serves as a learning resource for understanding how to work with vectors in Rust.

## Vector Operations Demonstrated

This project showcases the following vector operations:

1. **Creating vectors** - Using both `Vec::new()` and the `vec!` macro
2. **Adding elements** - Using the `push()` method
3. **Accessing elements** - Using indexing `[]` and the `get()` method
4. **Modifying elements** - Changing values in a mutable vector
5. **Iterating over vectors** - Both immutable and mutable iteration
6. **Vector properties** - Checking length with `len()`
7. **Removing elements** - Using `pop()` to remove the last element
8. **Safe access patterns** - Demonstrating how to safely access elements without panicking

## How to Run

To run this project, you need to have Rust installed on your system. Then:

1. Clone this repository or navigate to the project directory
2. Run the following command:

```bash
cargo run
```

This will compile and execute the program, displaying the output of various vector operations.

## Vec vs vec! - Understanding the Difference

### `Vec<T>` (Type)

`Vec<T>` is a struct type in Rust's standard library that represents a growable array type.

Example from the code:

```rust
let mut v: Vec<i32> = Vec::new();
v.push(5);
v.push(6);
```

This creates an empty vector that can store `i32` values, then adds elements to it one by one.

### `vec!` (Macro)

`vec!` is a macro that provides a convenient way to create a new vector with initial values.

Example from the code:

```rust
let mut v2 = vec![1, 2, 3, 4, 5];
```

This creates a vector with five elements already inside it. The macro handles the creation and population of the vector in a single step.

The `vec!` macro can also create vectors with repeated values:

```rust
// Not in the demo code, but a useful example
let zeros = vec![0; 10]; // Creates [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

## Prerequisites

To understand and run this project, you should have:

1. **Rust installed** - Visit [rust-lang.org](https://www.rust-lang.org/tools/install) for installation instructions
2. **Basic Rust knowledge** - Understanding of variables, types, and functions in Rust
3. **Cargo knowledge** - Basic familiarity with Rust's package manager and build system

## Key Concepts to Understand

1. **Ownership and borrowing** - The demo shows how to work with references to vector elements
2. **Mutability** - Demonstrates the difference between mutable and immutable vectors
3. **Error handling** - Shows how to safely access elements using pattern matching with `Option<T>`
4. **Iteration** - Demonstrates different ways to iterate through vectors

## Further Learning

After exploring this demo, you might want to look into:

- Other vector methods like `insert()`, `remove()`, and `extend()`
- Performance considerations when working with vectors
- Using vectors with custom types and structs
- Implementing traits for types that will be stored in vectors
