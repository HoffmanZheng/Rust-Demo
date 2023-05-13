# Variables

1. The varibles in Rust is immutable be default, which means the value of the variable cannot be assigned twice. Make it easy to develop the complicated and concurrent code.

2. Many factors need to consider when deciding a variable is mutable or not. Mpdifying a heavy object might be more efficient than assigning a new instance. On the other hand, it's quite comprehensible to use functional programing (assign value on new variable) when using lightweight object.

```
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

3. The new declared variable shadows the variable has the same name. Unlike using the `mut`, the shadow mechanism keep the variable immutable after the modification.

4. `let tup: (i32, f64, u8) = (500, 6.4, 1); ` Unlike the tuple, the elements in array `let a = [1, 2, 3, 4, 5];` are the same type. Once the length of the tuple or array is specified, it cannot be changed again.

