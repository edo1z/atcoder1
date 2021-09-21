use std::io;

fn main() {
    let numbers:Vec<i64> = get_input_i64();
    let a = numbers[0];
    let b = numbers[1];
    if 0 < a && b == 0 {
        println!("Gold");
    } else if a == 0 && 0 < b {
        println!("Silver");
    } else if 0 < a && 0 < b {
        println!("Alloy");
    }
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words:Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

fn get_input_i64() -> Vec<i64> {
    let words = get_input();
    words.iter().map(|word| word.parse().unwrap()).collect()
}
