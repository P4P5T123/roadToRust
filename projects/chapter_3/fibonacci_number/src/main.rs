fn main() {
    let number = 7;
    let result = fibonacci(number);
    println!("{number}th fibonacci number is: {result}");
}
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        x => fibonacci(x - 1) + fibonacci(x - 2),
    }
}