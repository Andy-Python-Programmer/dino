//! ## Basic Database
//! 
//! ```rust
//! // Create the database instance
//! let mut db = Database::new("./basic.dino");
//!
//! // Load and create the database if does not exist
//! db.load();
//!
//! // Insert values in the db in the format of key, value
//! db.insert("key-1", "value-1");
//! db.insert("key-2", "value-2");
//! ```
//! 
//! ## Sub Trees
//! 
//! ```rust
//! 
//! // Create the database instance
//! let mut db = Database::new("./sub_trees.dino");
//! 
//! // Load and create the database if does not exist
//! db.load();
//! 
//! // Create a new sub Tree in the main Tree of the db
//! let mut data_tree = Tree::new();
//! 
//! // Insert the key and value in the sub tree
//! data_tree.insert("a", "b");
//! 
//! // Insert the [data_tree] under the main tree
//! db.insert_tree("id", data_tree);
//! ```
//! 
//! ## Querying the Database
//! ```rust
//! // Create the database instance
//! let mut db = Database::new("./basic.dino");
//! 
//! // Load and create the database if does not exist
//! db.load();
//!
//! // Insert values in the db in the format of key, value
//! db.insert("key-1", "value-1");
//! 
//! // Print the value of `key-1`
//! println!("The value of key: id is {}", db.find("key-1").unwrap());
//! 
//! match db.find("not_exists") {
//!     Ok(_value) => {
//!         println!("This is unfortunate :(")
//!     }
//!
//!     Err(error) => {
//!         println!("Everting works! Here is the error for reference: {}", error)
//!     }
//! }
//! 
//! ```
//! ## Querying the Database
//! ```rust
//! // Create the database instance
//! let mut db = Database::new("./basic.dino");
//! 
//! // Load and create the database if does not exist
//! db.load();
//!
//! // Insert values in the db in the format of key, value
//! db.insert("id", "value-1");
//! 
//! // Remove a key in the database with its value
//! db.remove("id");
//!
//! // Now here it wont print that it exists as it does not we removed it ^^^^^
//! if db.contains_key("id") {
//!     println!("The key `id` exists!")
//! };
//!
//! println!("The length of items in the database is: {}", db.len());
//! ```

use std::fs::{ OpenOptions, File };
use std::io::SeekFrom;
use std::io::prelude::*;

/// The main struct of Dino.
/// The [Database] struct is responsible for creating the storage instance
/// that will store this database's documents, managing the database
/// tables as well as providing access to the default table.

pub struct Database {
    /// The path of the file in a [String] format
    pub path: String,

    /// The File object that we get when we open the database file
    file: Option<File>,

    /// The raw data in the file
    data: Option<String>,
    
    /// The json value of the file. Dino uses Json in backend to parse the database
    json: Option<serde_json::Value>
}

impl Database {
    /// Create a new instance of the [Database]
    pub fn new(path: &str) -> Database {
        return Database {
            path: String::from(path),
            file: None,
            data: None,
            json: None
        }
    }

    /// Load the database from the file and initialize variables
    pub fn load(&mut self) {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path.to_string())
            .unwrap();
        
        let mut buf = String::new();

        file.read_to_string(&mut buf).unwrap();

        let json = serde_json::from_str(if buf == "" { "{}" } else { buf.as_str() }).unwrap();

        self.file = Some(file);
        self.data = Some(buf);
        self.json = Some(json);
    }

    /// Insert a key with a subtree in the database
    pub fn insert_tree(&mut self, key: &str, value: Tree) {
        self.truncate();

        self.json.as_mut().unwrap().as_object_mut().unwrap().insert(key.to_string(), serde_json::from_str(value.children.unwrap().to_string().as_str()).unwrap());
        self.file.as_mut().unwrap().write(self.json.as_ref().unwrap().to_string().as_bytes()).expect("Cannot write to the database!");
    }

    /// Insert a key and a value in the database
    pub fn insert(&mut self, key: &str, value: &str) {
        self.truncate();
        
        self.json.as_mut().unwrap().as_object_mut().unwrap().insert(key.to_string(), serde_json::json!(value));
        self.file.as_mut().unwrap().write(self.json.as_ref().unwrap().to_string().as_bytes()).expect("Cannot write to the database!");
    }

    /// Remove a key in the database with its value
    pub fn remove(&mut self, key: &str) {
        self.truncate();
        
        self.json.as_mut().unwrap().as_object_mut().unwrap().remove(key);
        self.file.as_mut().unwrap().write(self.json.as_ref().unwrap().to_string().as_bytes()).expect("Cannot write to the database!");
    }

    /// Private function but is very important. 
    /// This truncates the db before we write the json code again
    fn truncate(&mut self) {
        self.file.as_ref().unwrap().set_len(0).unwrap();
        self.file.as_ref().unwrap().seek(SeekFrom::Start(0)).unwrap();
    }

    /// Find a value in the db
    pub fn find(&self, key: &str) -> Result<&serde_json::Value, String> {
        let val = &self.json.as_ref().unwrap()[key];

        if val == &serde_json::Value::Null {
            return Err(format!("The key `{}` does not exist in the database. You might want to create this or handle the error!", key))
        }

        return Ok(val);
    }

    /// Check if the key exists in the database
    pub fn contains_key(&mut self, key: &str) -> bool {
        return self.json.as_mut().unwrap().as_object_mut().unwrap().contains_key(key);
    }

    /// Return the length of items that are in the databse
    pub fn len(&mut self) -> usize {
        return self.json.as_mut().unwrap().as_object_mut().unwrap().len();
    }
}

/// The struct that allows you to create sub trees in the main tree in the database
/// Sub trees do not auto insert in the main tree of the database
/// You can do that by doing
/// # Example
/// ```rust
/// // Create the database instance
/// let mut db = Database::new("./hello.dino");
/// 
/// // Load and create the database if does not exist
/// db.load();
///
/// // Create a new sub Tree in the main Tree of the db
/// let mut data_tree = Tree::new();
///
/// // Insert the [data_tree] under the main tree
/// db.insert_tree("id", data_tree);
/// ```
/// Where the key always need to be a [String]
pub struct Tree {
    pub children: Option<serde_json::Value>
}

impl Tree {
    /// Create a new sub tree
    pub fn new() -> Tree {
        return Tree {
            children: serde_json::from_str("{}").unwrap()
        }
    }

    /// Insert data in the sub tree
    pub fn insert(&mut self, key: &str, value: &str) {
        self.children.as_mut().unwrap().as_object_mut().unwrap().insert(key.to_string(), serde_json::Value::String(value.to_string()));
    }
}