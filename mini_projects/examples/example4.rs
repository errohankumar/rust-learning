
enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn main() {
    let rect: Shape = Shape::Rectangle(20.0, 10.0);
    println!{"Area is {}",calculate_area(rect)};
    let circle: Shape = Shape::Circle(30.3);
    println!{"Area is {}", calculate_area(circle)};
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}