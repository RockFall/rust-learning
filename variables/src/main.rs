fn functional_add(x :i32, y :i32) -> i32 {
    return x + y
}

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = functional_add(x, 1);
    println!("The value of x is: {x}");

    const Y :i8 = 127;
    // Create variable 'y' that turn Y into i16 and adds 32 to it
    let y = Y as i16 + 32;

    println!("The value of y is: {y}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;

    println!("The values of a, b, c are: a={a}, b={b}, c={c}");
    println!("The value of tup is: ({}, {}, {})", tup.0, tup.1, tup.2);

    let arr = [1, 2, 3, 4, 5];
    println!("The value of arr is: [{}, {}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3], arr[4]);
}