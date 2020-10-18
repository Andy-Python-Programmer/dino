extern crate dino;

use dino::*;

fn main() {
    let mut db = Database::new("./hello.dino");
    db.load();

    db.insert("key", "q");

    let mut data_tree = Tree::new();
    data_tree.insert("b", "c");

    db.insert_tree("id", data_tree);

    db.find("a");

    println!("The value of key: id is {}", db.find("id"));
}