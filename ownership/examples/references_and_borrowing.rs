fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; this will not work here because println has r1 and r2

    println!("{r1}, {r2}"); // here the scope of r1 and r2 ends

    let r3 = &mut s; // no problem

    println!("{r3}");
}