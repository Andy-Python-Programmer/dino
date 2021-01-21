# Dino

![Crates.io](https://github.com/Andy-Python-Programmer/dino/workflows/Build/badge.svg)
![Crates.io](https://img.shields.io/crates/d/dino)
![Crates.io](https://img.shields.io/crates/v/dino)
![Crates.io](https://docs.rs/dino/badge.svg)

`Dino` is a lightweight database for rust!
It makes writing databases with types more easy.
Normally if you use a File Storage database then you will have to parse the types by yourself

Dino uses json ie. You will have all of the basic types like [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), etc... and the special one is Tree.
Dino is special about trees. Because some of the databases allow you to open trees but you cannot open sub trees there.
You can P easily open a sub tree. Here are some of the examples that show how to use trees and how to use the database without sub trees ;)

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

### Using it with [rocket.rs](https://crates.io/crates/rocket)

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

There is a lot more for you to explore! So check out the [docs](https://docs.rs/dino/0.1.2/dino/index.html) and the [examples](https://github.com/Andy-Python-Programmer/dino/tree/master/examples) directory.

## Contributing
Contributions are always welcome! Here are some ways you can contribute to dino:

1. File issues and buggs
2. Help us with documentation
3. Add new features in dino and fix buggs!

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
