use::chrono:: {Local, Utc};

fn main() {
    let now = Local::now();
    let utc_time = Utc::now();
    println!("Local Time is {} and UTC Time is {}", now, utc_time);
}
