use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    isbn: String,
}

fn main() {
    let opts = Args::parse();
    println!("The provided ISBN-10 is: {}", opts.isbn);

    if validate_isbn(&opts.isbn) {
        println!("The provided ISBN-10 is valid.");
    } else {
        println!("The provided ISBN-10 is invalid.");
    }
}

fn validate_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");
    if isbn.len() != 10 {
        return false;
    }

    let mut chars: Vec<char> = isbn.chars().collect();
    let check_char = chars.pop().unwrap();

    if !check_char.is_digit(10) && check_char != 'X' {
        return false;
    }
    if !chars.iter().all(|c| c.is_digit(10)) {
        return false;
    }

    let check_digit = if check_char == 'X' {
        10
    } else {
        check_char.to_digit(10).unwrap()
    };

    let sum: u32 = chars
        .iter()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(i, digit)| (10 - i as u32) * digit)
        .sum();

    (sum + check_digit) % 11 == 0
}
