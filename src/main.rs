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

    let isbn: Vec<u32> = isbn.chars().map(|c| c.to_digit(10).unwrap_or(10)).collect();

    let sum: u32 = isbn
        .iter()
        .enumerate()
        .map(|(i, &digit)| (10 - i as u32) * digit)
        .sum();

    sum % 11 == 0
}
