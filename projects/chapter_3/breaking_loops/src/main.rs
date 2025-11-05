fn main() {
    let mut counter = 0;
    'counting_up: loop {
        println!("Count = {counter}");
        let mut remaining = 10;
        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;    
        };
        counter += 1;
    }
    println!("End count is {counter}");
    liftoff();
}

fn liftoff() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}