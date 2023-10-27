use std::cmp::Ordering;
fn main() {
    enum BookFormat {
        Paperback,
        Hardback,
        Ebook,
    }

    struct Book {
        isbn: i32,
        format: BookFormat,
    }

    impl PartialEq for Book {
        fn eq(&self, other: &Self) -> bool {
            self.isbn == other.isbn
        }
    }

    impl PartialOrd for Book {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.isbn.cmp(&other.isbn))
        }
    }

    let b1 = Book {
        isbn: 3,
        format: BookFormat::Paperback,
    };
    let b2 = Book {
        isbn: 3,
        format: BookFormat::Ebook,
    };
    let b3 = Book {
        isbn: 10,
        format: BookFormat::Paperback,
    };

    assert!(b1 == b2);
    assert!(b1 != b3);
    assert!(b3 > b1);
}
