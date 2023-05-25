# Functions

1. The type of the method parameters must be declared, whereby the compiler could know your intent without the deduction from other codes.

2. The statements don't return any values, rather the expressions calculate and generate a value. If a semicolon is added in the end of a expression `x + 1`, it becomes a statement `x + 1;` and do not return a value.

3. The type of the method return values must be declared after an arrow (`->`). Most functions return the value of the last expression implicitly, use the `return` keyword if the value need to be returned early.

4. Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. The condition in `if` expression must be a `bool`, otherwise a `mismatched types` error will be raised.

5. The result type of each `if` arms must be the same if the outcome of the `if else` expression is assigned to a variable. Rust needs to know the outcome type at compile time, thus everywhere the variable is used could be verified. If the variable type is only determined at runtime, the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.

6. The value added after the `break` will be returned out of the loop. A loop label `'label_name: loop` could be used if we want to `break` or `continue` the specified loop, like `break 'label_name`.
