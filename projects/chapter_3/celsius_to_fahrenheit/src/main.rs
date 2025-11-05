fn main() {
    let result = celsius_to_fahrenheit(36.0);
    println!("{result} grad und es wird noch heisser, mach den Beat nie wieder leiser!");
    let result = fahrenheit_to_celsius(result);
    println!("{result} grad und es wird noch heisser, mach den Beat nie wieder leiser!");

}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0/5.0 + 32.0
}
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0/9.0
}