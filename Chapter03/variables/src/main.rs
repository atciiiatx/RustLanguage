const DAYS_PER_WEEK: u32 = 7;
const WEEKS_PER_YEAR: u32 = 52;
const DAYS_PER_YEAR: u32 = 365;
const PI: f64 = 3.14159;
const DEGREE_RANGE: u32 = 360;
const DEGREES_TO_RADIANS: f64 = PI / 180.0;

fn main() {
    println!("MUTABLE");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("CONSTANTS");
    println!("There are {} days in a week.", DAYS_PER_WEEK);
    println!("There are {} weeks in a year.", WEEKS_PER_YEAR);
    println!("There are {} days in a year.", DAYS_PER_YEAR);
    println!("The value of PI is approximately {}.", PI);
    println!("There are {} degrees in a full circle.", DEGREE_RANGE);
    println!(
        "To convert degrees to radians, multiply by {}.",
        DEGREES_TO_RADIANS
    );
    let angle_in_degrees: f64 = 45.0;
    let angle_in_radians = angle_in_degrees * DEGREES_TO_RADIANS;
    println!(
        "{} degrees is equal to {:.2} radians.",
        angle_in_degrees, angle_in_radians
    );

    println!("SHADOWING");
    let number = "3.14159";
    let number: f64 = number.parse().expect("Failed to parse string to f64");
    println!("The value of PI is approximately {:.5}.", number);
}
