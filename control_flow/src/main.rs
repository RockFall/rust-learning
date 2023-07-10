fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Match example
    let number = 3;

    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        // _ => println!("anything"),
        _ => (),
    }

    let condition: bool = false;
    let number: i32 = {let a: i32 = if condition { 15 } else { 31 }; a + 1};

    println!("The value of number is: {}", number);

    // Loops
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 21 {
            break counter * 2
        }
    };

    println!("The result is: {}", result);

    println!("Breaking inside innerloop, expect 10: {}", {
        let mut i = 0;
        'outer: loop {
            i += 1;
            'inner: loop {
                if i >= 10 {
                    break 'outer 10;
                }
                else {
                    break 'inner;
                }
            }
        }
    });

    // While loop
    let mut number: i32 = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    // For loop
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // For loop with range
    for number in 1..4 {
        println!("{}!", number);
    }

}
