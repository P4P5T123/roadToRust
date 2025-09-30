fn main() {
    println!("Hello, world!");
    print_labeled_measurement(5, 'h');

    let x = increment(5);
    println!("The value of x is: {x}");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn increment(x: i32) -> i32 {
    x + 1
}