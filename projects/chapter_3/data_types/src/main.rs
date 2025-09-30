fn main() {
    let x = 2.0;
    let y: f32 = 3.0;
    let sum = x + y;
    let difference = 42.2 - 3.2;
    let product = 4.2 * 90.23;
    let truncated = -5 / 3;
    let remainder = 42 % 5;
    println!("sum is {sum}, difference is {difference}, product is {product}, truncated is {truncated} and remainder is {remainder}");


    let tuple: (u8, f64, i32) = (8, 24.23, 3000);
    let (x, _, _) = tuple;
    println!("The first value of the tuple is {x}. ");
    println!("The second value of the tuple is {0}. ", tuple.1);


    let array = [0.0; 5];
    let array = [1, 2, 3, 4, 5];
    let mut output = "Here are the elements of the array:".to_owned();
    for i in 0..array.len() {
        let element: &str = &array[i].to_string();
        output.push_str(" ");
        output.push_str(element);
    };
    println!("{}", output);
}
