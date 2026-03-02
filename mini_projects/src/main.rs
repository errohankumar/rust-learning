use std::io;
use ibig::UBig;

fn convert(temperature: f32) -> f32 {
    temperature * 1.8_f32 + 32.0_f32
}


fn fibonacci(n: u32) -> UBig {
    if n == 0 {
        return UBig::from(0u8);
    } else if n == 1 {
        return UBig::from(1u8);
    }
    let mut a = UBig::from(0u8);
    let mut b = UBig::from(1u8);
    for _ in 2..=n {
        let temp = b.clone();
        b = a + &b;
        a = temp;
    }
    b
}

fn main() {
    let mut input = String::new();

    println!("Enter temperature in Celsius:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let temperature: f32 = match input.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Please enter a valid number");
            return;
        }
    };

    let temp_in_fahrenheit = convert(temperature);

    println!("Temperature in Fahrenheit is {}", temp_in_fahrenheit);


    let mut fib_input = String::new();

    println!("Enter n to get nth Fibonacci number");

    io::stdin()
        .read_line(&mut fib_input)
        .expect("Failed to read line!");

    let n: u32 = match fib_input.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Please enter a valid positive integer");
            return;
        }
    };

    let start = std::time::Instant::now();
    let fib_n = fibonacci(n);
    let duration = start.elapsed();
    println!("The {n}th Fibonacci number is {fib_n}");
    println!("Rust Fibonacci calculation took {:.8} seconds", duration.as_secs_f64());
}
