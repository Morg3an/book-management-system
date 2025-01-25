pub fn add_book(collection: &mut Vec<(String, String)>, title: &str, author: &str) {
    collection.push((title.to_string(), author.to_string()));
    println!("Book added successfully!");
}

pub fn list_books(collection: &Vec<(String, String)>) {
    if collection.is_empty() {
        println!("No books in the collection.");
    } else {
        for (i, (title, author)) in collection.iter().enumerate() {
            println!("{}. {} by {}", i + 1, title, author);
        }
    }
}

pub fn search_books(collection: &Vec<(String, String)>, query: &str) {
    let results: Vec<_> = collection
        .iter()
        .filter(|(title, author)| title.contains(query) || author.contains(query))
        .collect();

    if results.is_empty() {
        println!("No books found matching '{}'.", query);
    } else {
        for (title, author) in results {
            println!("{} by {}", title, author);
        }
    }
}

pub fn delete_book(collection: &mut Vec<(String, String)>, title: &str) {
    let initial_len = collection.len();
    collection.retain(|(book_title, _)| book_title != title);

    if collection.len() < initial_len {
        println!("Book '{}' deleted successfully!", title);
    } else {
        println!("No book found with the title '{}'.", title);
    }
}