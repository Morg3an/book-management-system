# Book Management System

## Overview

This is a simple **Book Management System** built as part of Day 4 of the 30-Day Rust Challenge. It demonstrates the use of **vectors**, Rust's dynamic array type, to store, manage, and manipulate a collection of books.  

## Features

- **Add Books**: Add new books to the collection by providing a title and author.
- **List Books**: View all books currently in the collection.
- **Search Books**: Search for books by title or author.
- **Delete Books**: Remove a book from the collection by its title.

## Concepts Covered

- Using **vectors** to store and manage data.
- Leveraging Rust's standard library for common operations.
- Structuring code with **modules** for clarity and reusability.
- Implementing interactive command-line programs.

## How to Run

1. Clone the repository:
   ```bash
   git clone https://github.com/Morg3an/book-management-system.git
   cd book-management-system
   ```
2. Buiild the project:
    ```bash
    cargo build
    ```
3. Run the project:
    ```bash
    cargo run
    ```

## Example usage
    ```mathematica
    1. Add Book
    2. List Books
    3. Search Books
    4. Delete Book
    5. Exit
    Enter your choice: 1
    Enter title: The Great Gatsby
    Enter author: F. Scott Fitzgerald
    Book added successfully!

    Enter your choice: 2
    1. The Great Gatsby by F. Scott Fitzgerald
    ```


## Technologies Used
- Rust
- Cargo (Rust's package manager)

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
