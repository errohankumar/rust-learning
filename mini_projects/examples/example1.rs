use std::io;

fn main() {
    let num: i32 = 5;

    if is_even(num) {
        println!("Numebr is Even");
    }
    else {
        println!("Numebr is odd")
    }
    
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    else {
        return false;
    }
}