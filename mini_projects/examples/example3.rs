fn main() {
    let name = String::from("Rohan");
    let length = get_str_len(&name);
    println!("Lenght is {}", length)
}

fn get_str_len(str: &str) -> usize {

    str.chars().count()

}