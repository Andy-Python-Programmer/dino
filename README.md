# Dino

![Crates.io](https://img.shields.io/crates/d/dino)
![Crates.io](https://img.shields.io/crates/v/dino)

## Example Code

```rs
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

// Insert the [data_tree] under the main tree
db.insert_tree("id", data_tree);

// Print the value of id
println!("The value of key: id is {}", db.find("id"));
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.