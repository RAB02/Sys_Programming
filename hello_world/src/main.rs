// use std::fs::OpenOptions;
// use std::io::Write;

// fn append_to_file() {
//     let mut file = OpenOptions::new()
//         .append(true) 
//         .open("example.txt")
//         .unwrap();

//     writeln!(file, "This line is appended to the file.").unwrap();
// }

// use std::process::Command;

// fn executing_os_commands_linux() {
//     let output = Command::new("sudo")
//         .arg("cat")
//         .arg("example.txt")
//         .output()
//         .expect("Failed to execute command");

//     println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
// }
// fn executing_os_commands_linux() {
//     let output = Command::new("python3")
//         .arg("my_script.py")
//         .output()
//         .expect("Failed to execute command");

//     println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
// }

// fn main() {
//     executing_os_commands_linux();
// }

use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).unwrap();
    for book in books{
        writeln!(file, "{}, {}, {}", book.title, book.author, book.year).unwrap();
    }
      
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).unwrap();
    let buf_reader = BufReader::new(file);
    let data: Vec<Book> = vec![];

    file.read_to_string(&mut data).unwrap();


    return data;
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}