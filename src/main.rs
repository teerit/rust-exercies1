fn main() {
    let mut library = Library::new();

    println!("The library is empty: library.is_empty() -> {}", library.is_empty());
    
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    
    println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());
    
    
    library.print_books();
    
    match library.oldest_book() {
       Some(book) => println!("The oldest book is {}", book.title),
       None => println!("The library is empty!"),
    }
    
    println!("The library has {} books", library.len());
    library.print_books();
}

struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Update the `self` parameter to
// indicate the method's required level of ownership over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {

    fn new() -> Library { // No receiver, a static method
        Library { books: Vec::new() }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) { // Exclusive borrowed read-write access to self
        self.books.push(book);
    }

    fn print_books(&self) {
        for (idx, book) in self.books.iter().enumerate() {
            let title = &book.title;
            let year = &book.year;
            println!("Book number {idx}: title {title} year {year}");
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        let mut oldest: Option<&Book> = None;
        for book in self.books.iter() {
            if oldest.is_none() || book.year < oldest.unwrap().year {
                oldest = Some(book);
            }
        }

        oldest
    }
}

#[test]
fn test_library_len() {
    let mut library = Library::new();
    assert_eq!(library.len(), 0);
    assert!(library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    assert_eq!(library.len(), 1);
    assert!(!library.is_empty());
}

#[test]
fn test_library_print_book() {
    let mut library = Library::new();
    library.add_book(Book::new("Lord of the Rings", 1954));
    assert_eq!(library.print_books(), ());
}