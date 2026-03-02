fn main() {
    //one();
    //two();
    //three();
    four();
}


fn one() {
    let mut number = 10;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn two() {
    let a = [10, 20, 30, 40, 50, 60];
    let mut index = 0;

    while index < 6 {
        println!("the value is : {}", a[index]);
        index += 1;
    }
}

fn three() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", {element});
    }
}

fn four() {
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!");
}
