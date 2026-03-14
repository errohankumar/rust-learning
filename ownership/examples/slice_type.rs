fn first_word(s: &String)-> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn second_word(s: &String) -> (usize, usize) {
    
}

fn main() {
    let s = String::from("Iam Rohan Kumar");
    let f_word = first_word(&s);
    println!("{f_word}")
}
