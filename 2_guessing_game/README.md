# Guessing game

1. `let mut guess = String::new();` The `::` syntax indicates that `new` is an associated function of the `string` type, known as the static function in other language.

2. `io::stdin().read_line(&mut guess).expect("Failed to read line");` The `&` indicates that the argument is a reference, which allows data access without duplicating the variables. The `read_line` method returns an `io::Result` value, which is an enum. If the result instance is an `Err`, program will crash and the message (as the parameter in `expect` method) will prompt.

```
[dependencies]
rand = "0.8.5"
```
3. Includes the library crate in `Cargo.toml`, thereby it could be used as dependency in the project. The semantic versioning is used here, `0.8.5` is actually shorthand for `^0.8.5`, indicates the versions which is at least 0.8.5 and API compatible with 0.8.5.

4. Cargo checks whether the dependencies have been downloaded and compiled, and whether there is any changes in the source code or Cargo.toml, aviods the duplicated work and exits quickly.

```
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

5. A `match` expression consists of many pattern-containing arms, the corresponding logic would be executed if the given value matches a pattern.

```
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

6. The underscore `_` is a wildcard, which could match any possible Err values, no matter what kind of error info it contains.
