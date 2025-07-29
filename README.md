# learn-rust
🦀 This repository is a collection of practical Rust programming exercises, examples, and notes

## Why use Rust?
Rust is a programming language that has an extremely strict type system.
Rust Goals:-
- Speed: Rust's binaries are as fast as C binaries, sometimes outpacing C++ binaries!
- Memory safety: Rust has a huge emphasis on memory safety.
- Concurrency: Focusing on memory safety eliminates a lot of race condition-like scenarios and helps you introduce concurrency in your program.

Following are a few errors mistakes one might make in languages like C/C++ (but not with Rust):

- Use after free
- Double free
- Accessing out-of-bound values
- Using ```NULL```
- Inappropriate pointer arithmetic and/or access
- Use of uninitialized variable(s)
- Thread-unsafe multi-threading

## Install Rust 🦀
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Variables
Every variable that you declare is **immutable by default**. This means that once a value is assined to the variable,it can be changed. This ensures that it won't make special provisions like *spin locks or mutexes* to introduce multi-threading.

Variables can be explicitly defined to allow mutation they are called mutable variables.

```
// immutable variable
let variable_name1 = value1;

// mutable variable
let mut variable_name2 = value2;
```

## Links

### IT'S FOSS Rust Tutorial 

```
https://itsfoss.com/rust-tutorials/
```

### Let's Get Rusty
```
https://learn.letsgetrusty.com/index.html
```
