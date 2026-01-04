# R-Agent

R-Agent is an experimental project designed to showcase and test the capabilities of my custom library, `forge`. This
library is currently under development as part of the [Pokebrains](https://github.com/ronakgh97/Pokebrains) project.

## Features

- Demonstrates the integration of `forge` in a real-world application.
- Supports piping, regex, and all traditional Unix operations to maintain an old-school, not modern slop.

## Command-Line Arguments

R-Agent supports the following command-line arguments:

- **`--config <file_name>`**: Specifies the configuration file to use. This file contains settings and parameters for
  the
  agent.
- **`--session <name>`**: (Optional) Defines the session name. Sessions allow you to maintain context across multiple
  commands.
- **`--image <path/url>`**: (Optional) Provides an image path or URL for tasks that require visual input.
- **`<task>`**: The task or command you want the agent to perform.

## Usage

```bash
cat Cargo.toml | ragent run "explain the dependencies" --config qwen_qwen3-coder-free.toml --session my_session

rg "TODO" | ragent run "explain the todos" --config qwen_qwen3-coder-free.toml --session my_session
````

## Little DEMO

````shell
ronak in r-agent on   master 
❯ cargo check 2>&1 | ragent run "explain me the error and cause" --config zai-org_glm-4.6v-flash                    

Running agent...

Task: explain me the error and cause
Config: zai-org_glm-4.6v-flash
Image: None
Session: None
Context: 569 chars

This is a classic Rust borrowing rule violation. Let me explain:

## What the Error Means

The error `error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable` occurs when you try to
have both:
1. An **immutable reference** (`&T`)
2. A **mutable reference** (`&mut T`)

active at the same time for the same piece of data.

## The Specific Problem in Your Code

In your `main.rs`, line 46 creates an immutable reference:
```rust
let r1 = &s;
```

Then line 47 tries to create a mutable reference to the *same* variable:
```rust
let r2 = &mut s;
```

This violates Rust's borrowing rules because:

- Once you have an immutable reference, no other references (including mutable ones) can be created until the immutable
  reference goes out of scope.
- The compiler ensures memory safety by preventing data races and other concurrency issues.

## How to Fix This

You'll need to either:
1. Remove one of the references before creating the other
2. Restructure your code so you don't need both types of references simultaneously

For example, if you want to print both values, consider:

```rust                                                                                                                                                            
let r1 = &s;                                                                                                                                                       
println!("{}", r1);                                                                                                                                                
// Then create and use the mutable reference                                                                                                                       
let mut s_mut = s; // or some other way to get a mutable version                                                                                                   
let r2 = &mut s_mut;                                                                                                                                               
println!("{} {}", r1, *r2); // dereference if needed                                                                                                               
```                                                                                                                                                                

The key takeaway is that Rust enforces strict borrowing rules to ensure memory safety and prevent data races.
````

## Note

`my_lib` is not yet published to crates.io. Frequent updates are made locally to ensure rapid iteration without the
constraints of versioning and publishing.

## License

This project is experimental and does not yet have a formal license.
