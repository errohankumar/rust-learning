struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let name = String::from("Rohan");
    let user = User {
        name,
        age: 30,
        active: true,
    };
    print!("{} is {} years old", user.name, user.age);
}