fn main() {
    let first_day: u8 = 0;
    let last_day: u8 = 11;
    let twelve_days_of_christmas = String::new();
    for day in first_day..last_day {
        twelve_days_of_christmas.push_stf(format!"{0}\n{1}", get_verse_start(&day), get_sendings(&day));
    };
    println!(twelve_days_of_christmas);
}
const ORDINALS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh", "twelveth"];
const SENDINGS: [&str; 12] = ["a partridge in a pear tree", "two turtle doves", "three French hens", "four calling birds", "five gold rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumg"];

fn get_verse_start(day: &u8) -> String {
    let idx = *day as usize;
    let ord = ORDINALS.get(idx).expect("day must be in 0..=11");
    format!("On the {ord} day of Christmas my true love sent to me")
}
fn get_sendings(day: &u8, sent: &str) -> String {
    let idx = *day as usize;
    let ord = SENDINGS.get(idx).expect("day must be in 0..=11");
    if idx == 0 {
        return ord.to_string();
    } else if idx == 1 {
        let first_day_verse = format!("{0}\n and {1}", ord, sendings[0]);
        return first_day_verse.to_string();
    }
    format!("{ord}")
}