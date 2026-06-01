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

### 9. [Structs](https://github.com/AlphonseRaph/The_Rust_Programming_Language_Book/blob/master/structs)

- **Path:** `structs/src/main.rs`
- **Description:** Covers Chapter 5 of the book — Structs. Demonstrates
  defining basic structs (e.g. User), tuple structs (e.g. Color, Point),
  adding methods with `impl`, associated functions as constructors,
  and debug printing with `{:?}`.

### 10. [Rectangles](https://github.com/AlphonseRaph/The_Rust_Programming_Language_Book/blob/master/structs/rectangles)

- **Path:** `structs/rectangles/src/main.rs`
- **Description:** A worked example from Chapter 5 — calculates the area
  of a rectangle using a struct. Demonstrates refactoring from plain
  variables to tuples to a named struct, and using `impl` to add
  an `area` method directly on the Rectangle struct.

### 11. [Enums and Pattern Matching](./enums_and_pattern_matching)

- **Path:** `enums_and_pattern_matching/src/main.rs`
- **Description:** Covers Chapter 6 of the book — Enums and Pattern Matching.
  Demonstrates defining enums with various types of associated data,
  implementing methods on enums, using the `Option` enum for null safety,
  and exhaustive pattern matching with `match`. Also covers `if let`,
  `let else`, and using `_` as a catch-all pattern.

### 12. [Packages, Crates, and Modules](./packages_crates_modules)

- **Path:** `packages_crates_modules/src/main.rs` and `packages_crates_modules/restaurant/src/lib.rs`
- **Description:** Covers Chapter 7 of the book — Managing Growing Projects with Packages, Crates, and Modules. Demonstrates organizing code using modules, controlling visibility with the `pub` keyword, using absolute and relative paths (including `super` and `crate`), and bringing paths into scope with the `use` keyword. It also covers re-exporting with `pub use`, using external packages, and the multi-file module system.

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
