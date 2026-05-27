# The Rust Programming Language Book - Exercises

This repository contains my progress and exercises as I work through "The Rust Programming Language" book. Each directory represents a specific topic or project from the book.

## Projects Overview

### 1. [Hello Cargo](./hello_cargo)
- **Path:** `hello_cargo/src/main.rs`
- **Description:** A basic "Hello, world!" program created using Cargo, Rust's build system and package manager.

### 2. [Guessing Game](./guessing_game)
- **Path:** `guessing_game/src/main.rs`
- **Description:** An interactive CLI game where the program generates a random number between 1 and 100, and the player tries to guess it. It demonstrates `std::io`, `rand` crate, `match` expressions, and loops.

### 3. [Variables and Mutability](./variables)
- **Path:** `variables/src/main.rs`
- **Description:** Explores Rust's core concepts of variable mutability, shadowing, and basic data types including scalars (integers, floats, booleans, characters) and compounds (tuples, arrays).

### 4. [Functions](./functions)
- **Path:** `functions/src/main.rs`
- **Description:** Demonstrates how to define functions with parameters and return values. It also highlights the distinction between statements and expressions in Rust.

### 5. [Branches (Control Flow)](./branches)
- **Path:** `branches/src/main.rs`
- **Description:** Covers basic control flow using `if`, `else if`, and `else` statements, including using `if` in a `let` statement.

### 6. [Loops](./loops)
- **Path:** `loops/src/main.rs`
- **Description:** Demonstrates various looping constructs: `loop`, `while`, and `for`. Includes examples of returning values from loops and using loop labels to disambiguate nested loops.

### 7. [Ownership](./ownership)
- **Path:** `ownership/src/main.rs`
- **Description:** Deep dive into Rust's most unique feature: ownership. Covers stack vs. heap memory, variable scope, the `String` type, moving, cloning, and how ownership works with functions.

### 8. [References and Borrowing](./references_and_borrowing)
- **Path:** `references_and_borrowing/src/main.rs`
- **Description:** Explores the concepts of references and borrowing, allowing data access without taking ownership. Covers immutable and mutable references, data race prevention, and string slices (&str).

## How to Run

To run any of the projects, navigate into its directory and use Cargo:

```bash
cd <project_directory>
cargo run
```

For example:
```bash
cd guessing_game
cargo run
```

---
*Learning Rust, one step at a time!*
