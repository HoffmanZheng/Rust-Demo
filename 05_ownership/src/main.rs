fn main() {
    // mutable string
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // move
    let s1 = String::from("hello");
    let s2 = s1;
    // error[E0382]: borrow of moved value: `s1`
    // println!("{}, world!", s1);

    // ownership and functions
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    // reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // change a reference
    let s = String::from("hello");
    change(&s);
    // error[E0596]: cannot borrow `*some_string` as mutable 

    // mutable references
    let mut s = String::from("hello");
    change2(&mut s);
    println!("Mutable reference after change: {}", s);

    // dangling references
    // let reference_to_nothing = dangle();

    // slice
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // error! cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("the first word is: {}", word);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

fn change(some_string: &String) {
    // some_string.push_str(", world");
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
