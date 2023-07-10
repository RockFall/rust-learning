fn main() {
    print_labeled_measurement(5, 'h');

    // Functional numbers
    println!("Zero: {zero}", zero = zero());
    println!("One: {one}", one = add_one(zero()));
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn zero() -> i32 {
    0
}

fn add_one(x: i32) -> i32 {
    x + 1
}