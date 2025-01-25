mod books;

use books::*;

fn main() {
    let mut collection: Vec<(String, String)> = Vec::new();

    loop {
        println!("1. Add Book");
        println!("2. List Books");
        println!("3. Search Books");
        println!("4. Delete Book");
        println!("5. Exit");

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter title:");
                let mut title = String::new();
                std::io::stdin().read_line(&mut title).unwrap();

                println!("Enter author:");
                let mut author = String::new();
                std::io::stdin().read_line(&mut author).unwrap();

                add_book(&mut collection, title.trim(), author.trim());
            }
            "2" => list_books(&collection),
            "3" => {
                println!("Enter search query:");
                let mut query = String::new();
                std::io::stdin().read_line(&mut query).unwrap();

                search_books(&collection, query.trim());
            }
            "4" => {
                println!("Enter title to delete:");
                let mut title = String::new();
                std::io::stdin().read_line(&mut title).unwrap();

                delete_book(&mut collection, title.trim());
            }
            "5" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}
