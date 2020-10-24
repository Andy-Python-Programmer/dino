# Dino

![Crates.io](https://github.com/Andy-Python-Programmer/dino/workflows/Build/badge.svg)
![Crates.io](https://img.shields.io/crates/d/dino)
![Crates.io](https://img.shields.io/crates/v/dino)

## Example Code

### Basic Database

```rust
// Create the database instance
let mut db = Database::new("./basic.dino");

// Load and create the database if does not exist
db.load();

// Insert values in the db in the format of key, value
db.insert("key-1", "value-1");
db.insert("key-2", "value-2");
```

### Sub Trees

```rust
// Create the database instance
let mut db = Database::new("./sub_trees.dino");

// Load and create the database if does not exist
db.load();

// Create a new sub Tree in the main Tree of the db
let mut data_tree = Tree::new();

// Insert the key and value in the sub tree
data_tree.insert("a", "b");

// Insert the [data_tree] under the main tree
db.insert_tree("id", data_tree);
```

### Querying the Database

```rust
// Create the database instance
let mut db = Database::new("./basic.dino");

// Load and create the database if does not exist
db.load();

// Insert values in the db in the format of key, value
db.insert("key-1", "value-1");

// Print the value of `key-1`
println!("The value of key: id is {}", db.find("key-1").unwrap());
```

<img src="dist/rocket.png" height="100px">

## Using [Dino](https://crates.io/crates/dino) with [Rocket](https://crates.io/crates/rocket)!

```rust
// Simple rocket route
#[get("/<id>")]
// Here we add the arg `db: State<dino::Database>`
// To get the db state that we passed before!
fn hello(db: State<dino::Database>, id: String) -> String {
    // Now in this rocket route we take a id param
    // We will extract the param to str
    // Then check if it exists
    match db.find(id.as_str()) {
        // If it exists it will return Ok(value)
        Ok(value) => {
            // Then we can return the value!
            return value.to_string();
        }

        // If it does not exists it gives a error
        Err(error) => {
            // So return the error!
            // You might want to handle the error too!
            return error;
        }
    }
}

fn main() {
    // Create the database instance
    let mut db = dino::Database::new("rocket.dino");

    // Load and create the database if does not exist
    db.load();

    // Insert a key with a dummy value for now!
    db.insert("key", "value!");

    // Ignite the rocket and mount the routes
    rocket::ignite()
        .mount("/", routes![hello])
        // Important part here!
        // Here we pass the db state to rocket
        // So we can access it when we go to any route.
        .manage(db)
        .launch();
}
```

There is a lot more for you to explore! So check out the [docs](https://docs.rs/dino/0.1.0/dino/) and the [examples](https://github.com/Andy-Python-Programmer/dino/tree/master/examples) directory.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
