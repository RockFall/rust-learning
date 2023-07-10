fn main() {
    let mut s1 = String::from("hello");
    reference_print(&s1); // borrow s1

    // We can't change s1 here because it's borrowed
    // unless we use mutable reference
    change(&mut s1);
    reference_print(&s1);

    // We can have multiple immutable references
    // but only one mutable reference
    let mut s2: String = String::from("hello");
    let _r1: &String = &s2;
    let _r2: &String = &s2;
    println!("{}, {}", _r1, _r2); // OK

    let _r3: &mut String = &mut s2;
    _r3.push_str(", from r3");
    //println!("{}, {}, and {}", _r1, _r2, _r3); // error (mutable borrow occurs)
    println!("{}", _r3);                         // valid (immutable borrows are over)


    // Slices
    let mut s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];
    println!("Slice [0..5] = {}, [6..11] = {}", _hello, _world);

    // First word
    let _first_word = first_word(&s);
    println!("First word: {}", _first_word);
    s.clear(); // error (immutable borrow occurs)


}

fn reference_print(s: &String) {
    println!("s: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() { // iter() returns each element in a collection
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}