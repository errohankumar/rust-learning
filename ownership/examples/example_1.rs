fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);  // s's value moves into the function

    // println!("{s}");  //this will cause it to fail because ownership has been taken away from s 

    let x = 5; // x comes into scope

    makes_copy(x); // Becasue i32 implements the Copy trait,
                   // x does not move into the function,
                   // so it is okay to use x afterward

    println!("{x}")
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
