# Hello Cargo

Cargo is the Rust's build-in build system and package manager.

```
cargo new <your_project_name>                  // create a new project using cargo
cd <your_project_name> && cargo build          // build the project
./target/debug/<your_project_name>             // run the executable under target/debug
cargo run                                      // compile the code and run the resultant executable
cargo check                                    // check the compilation error quickly without generating an executable
cargo build --release                          // compile with optimization
```

1. Cargo attempts to fetch the username and email from the local environment when creates a new project, the `Cargo.toml` could be edited if the info there is incorrect.

2. The compilation takes longer if it's in the release mode, the optimization let the Rust code run faster.
