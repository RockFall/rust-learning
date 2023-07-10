fn main() {

    // Inside a scope (between { and }), variables are valid 
    // from the point at which they’re declared until the end of the scope.
    {                            // s is not valid here, it’s not yet declared
        let _s = "hello";   // s is valid from this point forward
    }                            // this scope is now over, and s is no longer valid


    // drop() is called at the end of the scope 
    // for types that implement the Drop trait
    {
        let mut s: String = String::from("hello");
        s.push_str(" universe!\n");

        println!("s = {}", s);
    } // On the end of the scope, s is freed (drop(s))
    
    // Rust automatically copies values that have the Copy trait
    {
        let x: i32 = 5;
        let y: i32 = x; // x is copied to y

        println!("x = {}\ny = {}", x, y);
    }

    // Rust will never automatically create “deep” copies of your data,
    // except in the case of simple scalar values or when the Copy trait is implemented.
    {
        let s: String = String::from("hello");
        let mut s2: String = s; // s is moved to s2

        // s.push_str(" universe!"); INVALID because s is moved to s2
        //println!("{}", s); INVALID because s is moved to s2

        s2.push_str(" universe, from s2!\n");
        println!("s2 = {}", s2);

    } // On the end of the scope, s2 is freed (drop(s2))


    // If we do want to deeply copy the heap data of the String,
    // not just the stack data, we can use a common method called clone.
    {
        let mut s: String = String::from("hello");
        let mut s2: String = s.clone(); // s is cloned to s2

        s.push_str(" universe!");
        s2.push_str(" universe, from s2!\n");

        println!("s = {}\ns2 = {}", s, s2);
    }

    let _s1 = give_ownership();
    let s2 = String::from("hello");
    let _s3 = take_and_give_back(s2); // s2 is moved to take_and_give_back() and then moved to s3

}

fn give_ownership() -> String {
    let s: String = String::from("hello");

    s // s is returned and moved to the calling function
}

fn take_and_give_back(s: String) -> String {
    s // s is returned and moved to the calling function
}