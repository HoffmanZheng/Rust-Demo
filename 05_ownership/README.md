# Ownership

Ownership is the most unique feature of Rust, it enables memory safety guarantees without garbage collector. 

### Ownership

1. Unlike other programming languages that use garbage collection or explicitly allocate/free the memory, Rust manages memory using an ownership system with a set of rules.

2. The stack is referred as 'last in, first out' structure, it can only store the data with a known and fixed size. Allocating on the heap is slower than on the stack because the allocator must first find a big enough space to store the new data.

3. The Rust ownership rules: each value has an owner; there can only be one owner at a time; when the owner goes out of scope, the value will be dropped.

4. The immutable string literal is fast and efficient, because it's hardcoded directly into the final executable. A mutable and growable piece string is stored on the heap, the memory is automically returned once the its owner goes out of scope.

```
    let s1 = String::from("hello");
    let s2 = s1;

    // error[E0382]: borrow of moved value: `s1`
    println!("{}, world!", s1);
```

5. Rust invalidates the first variable when making a 'shallow copy'(for variables stored on the heap), known as `move`. Therefore the freeing memory would only occur when `s2` goes out of scope.

```
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

6. Passing a variable to a function / returning values will move or copy. It's quite annoying that anything we passed in also needs to be passed back if we want to use it again, Rust let us return multiple values using a tuple.

### References and Borrowing

```
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

1. The ampersands represent references, they allow you to refer to some value without taking ownership of it. The action is referred as borrowing, cannot be borrowed as mutable.

2. The mutable references enable the change in the calling functions. At any given time, you can have either one mutable reference or any number of immutable references. The benefit of having this restriction is that Rust can prevent data races at compile time.

3. Dangling references: the compiler ensures that the data will not go out of scope before the reference to the data does. 

### Slice

1. Returning the slice instead of returning the index of the string, the compiler will ensure the references into the string remain valid.
