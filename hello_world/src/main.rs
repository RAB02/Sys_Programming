
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
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);
    let mut data: Vec<Book> = vec![];

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(','); 
        let title = parts.next().unwrap().to_string();
        let author = parts.next().unwrap().to_string();     
        let year_str = parts.next().expect("Missing year").trim();
        let year = year_str.parse::<u16>().expect("Failed to parse year");
        
        data.push(Book { title, author, year });
    }
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

// #[derive(Debug)]
// enum GradeLevel {
//     Bachelor,
//     Master,
//     PhD,
// }
// #[derive(Debug)]
// enum Major {
//     ComputerScience,
//     ElectricalEngineering,
// }
// #[derive(Debug)]
// struct Student {
//     name:String,
//     grade:GradeLevel,
//     major:Major
// }

// impl Student {
//     fn new(name:String,grade:GradeLevel,major:Major) -> Self {
//         Student {
//             name:name,
//             grade:grade,
//             major:major,
//         }
//     }

//     fn introduce_yourself(&self) {
//         println!("My name is {}.", self.name);

//         let classMsg = match self.grade{
//             GradeLevel::Bachelor => "I am a Bachelors Student",
//             GradeLevel::Master => "I am a Masters Student",
//             GradeLevel::PhD => "I am a PHD Student"
//         };

//         let majorMsg = match self.major{
//             Major::ComputerScience => "That studies in Computer Science",
//             Major::ElectricalEngineering => "That studies in Electrical Engineering",
//         };

//         println!("{}", classMsg);
//         println!("{}", majorMsg);

//     }
// }
        

// fn main() {
//     let s1 = Student::new("John".to_string(),
//     GradeLevel::Bachelor,
//     Major::ComputerScience);
//     s1.introduce_yourself();
// }
