#[derive(Debug)]
struct Library {
    books: Vec<Book>,
}

#[derive(Debug)]
struct Book {
    title: String,
    year: u16,
}

impl Book {
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("{:#?}", book);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        let mut oldest: Option<&Book> = None;

        for book in &self.books {
            if oldest.is_none() || book.year > oldest.unwrap().year {
                oldest = Some(book);
            }
        }
        oldest
    }
}

fn main() {
    let mut library = Library::new();

    println!("The library is empty: {}", &library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!("The library is no longer empty: {}", &library.is_empty());

    library.print_books();

    match &library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());
    println!("The library has {:#?} books", library);
    library.print_books();
}

