fn main() {
    // let x = (let y = 6);  // the statement do not return values
    
    fn five() -> i32 {
        5
    }

    let y = plus_one(5);
    println!("The value of y is: {y}");

    // each arm of the if expression must be the same type
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; 
    println!("The value of number is: {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
	    break counter * 2;
        }
    };
    println!("The result returned from the loop: {result}");    

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn plus_one(x: i32) -> i32 {
    x + 1    // +;  ---> raise mismatched types error
}
