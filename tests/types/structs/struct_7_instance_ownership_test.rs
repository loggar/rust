#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    rating: f32,
    unix_release_date: i64,
}

impl Book {
    fn new(title: &str, author: &str, rating: f32, unix_release_date: i64) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            rating: rating,
            unix_release_date: unix_release_date,
        }
    }
}

#[test]
fn struct_instance_1_test() {
    let book = Book::new("The Catcher in the Rye", "J.D. Salinger", 4.5, 0);

    assert_eq!(book.title, "The Catcher in the Rye", "instance returns");
    assert_eq!(book.author, "J.D. Salinger", "instance returns");
    assert_eq!(book.rating, 4.5, "instance returns");
    assert_eq!(book.unix_release_date, 0, "instance returns");
}

#[test]
fn struct_instance_2_test() {
    let book1 = Book {
        title: String::from(
            "Learning Patterns: Patterns for Building Powerful Web Apps with Vanilla JavaScript and React",
        ),
        author: String::from("Lydia Hallie · Addy Osmani"),
        rating: 4.8,
        unix_release_date: 1633042800,
    };

    assert_eq!(
        book1.title, "Learning Patterns: Patterns for Building Powerful Web Apps with Vanilla JavaScript and React",
        "instance returns"
    );

    let mut book2 = Book {
        title: String::from("This is a placeholder book name."),
        author: String::from("This is a placeholder author."),
        ..book1
    };

    // At this point, book1.rating and book1.unix_release_date
    // have been moved to book2 and are no longer accessible
    assert_eq!(book2.rating, 4.8, "instance returns");
    assert_eq!(book2.unix_release_date, 1633042800, "instance returns");
    assert!(book2.title == "This is a placeholder book name.", "instance returns");

    book2.author = book1.author;
    assert!(book2.author == "Lydia Hallie · Addy Osmani", "instance returns");

    // Now book1.author has been moved to book2 as well
    // assert!(book1.author == "Lydia Hallie · Addy Osmani", "instance returns"); // borrow of moved value: `book1.author`
    // but book1.title is still accessible on book1
    assert!(
        book1.title == "Learning Patterns: Patterns for Building Powerful Web Apps with Vanilla JavaScript and React",
        "instance returns"
    );
}
