extern crate dino;

use dino::*;

fn main() {
    // Create the database instance
    let mut db = Database::new("./hello.dino");

    // Load and create the database if does not exist
    db.load();

    // Insert values in the db in the format of key, value
    db.insert("key", "q");

    // Create a new sub Tree in the main Tree of the db
    let mut data_tree = Tree::new();

    // Insert the key and value in the sub tree
    data_tree.insert("b", "c");

    // The length of items in the sub tree in the database
    // This also shows almost all of the functions in Database are also avaliable in Tree
    println!("The length of items in the sub tree in the database is: {}", data_tree.len());

    // Insert the [data_tree] under the main tree
    db.insert_tree("id", data_tree);

    // Print the value of id
    println!("The value of key: id is:\n{}", db.find("id").unwrap());

    match db.find("not_exists") {
        Ok(_value) => {
            println!("This is unfortunate :(")
        }

        Err(error) => {
            println!("Everting works! Here is the error for reference: {}", error)
        }
    }

    // Remove a key in the database with its value
    db.remove("id");

    // Now here it wont print that it exists as it does not we removed it ^^^^^
    if db.contains_key("id") {
        println!("The key `id` exists!")
    };

    println!("The length of items in the database is: {}", db.len());

    // Insert a number in the database
    db.insert_number("test", 1);
    db.insert_array("test-array", vec!["hello!"]);
    db.insert_bool("test-bool", true);

    println!("{}", db.find("test").unwrap().to_number() + 1); // This will print 2
    println!("{:?}", db.find("test-array").unwrap().to_vec()); // This will print ["hello!"]
    println!("{:?}", db.find("test-bool").unwrap().to_bool()); // This will print true
}