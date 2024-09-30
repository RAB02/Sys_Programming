use std::io::{self, Read, Write};
use std::fs::File;


#[derive(Debug)]
struct Car {
    model: String,
    year: u32,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    print!("How old are you? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();

    let car = Car { model, year };
    println!("Hi {}, you are {} years old!", car.model, car.year);

    let mut file = File::create("database.txt").unwrap();
    writeln!(file, "Model: {:?}", car.model).unwrap();
    writeln!(file, "Year: {:?}", car.year).unwrap();

    println!("{:?}", car);
    println!("Model: {:?}", car.model);
    println!("Year: {:?}", car.year);
    
}

fn main() {
    reading_from_console();
}