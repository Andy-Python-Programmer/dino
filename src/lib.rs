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
//! ## Basic Operations
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
use std::fmt;
use std::sync::Mutex;

/// The main struct of Dino.
/// The [Database] struct is responsible for creating the storage instance
/// that will store this database's documents, managing the database
/// tables as well as providing access to the default table.

pub struct Database {
    /// The path of the file in a [String] format
    pub path: String,

    /// The File object that we get when we open the database file
    file: Mutex<Option<File>>,

    /// The raw data in the file
    data: Mutex<Option<String>>,
    
    /// The json value of the file. Dino uses Json in backend to parse the database
    json: Mutex<Option<serde_json::Value>>
}

impl Database {
    /// Create a new instance of the [Database]
    pub fn new(path: &str) -> Database {
        return Database {
            path: String::from(path),
            file: Mutex::new(None),
            data: Mutex::new(None),
            json: Mutex::new(None)
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

        self.file = Mutex::new(Some(file));
        self.data = Mutex::new(Some(buf));
        self.json = Mutex::new(Some(json));
    }

    /// Insert a key with a subtree in the database
    pub fn insert_tree(&self, key: &str, value: Tree) {
        self.json.lock().unwrap().as_mut().unwrap().as_object_mut().unwrap().insert(key.to_string(), serde_json::from_str(value.children.unwrap().to_string().as_str()).unwrap());
        
        self.save_data();
    }

    /// Insert a key and a value in the database
    pub fn insert(&self, key: &str, value: &str) {        
        self.json.lock().unwrap().as_mut().unwrap().as_object_mut().unwrap().insert(key.to_string(), serde_json::json!(value));

        self.save_data();
    }

    /// Insert a key and a value in the database
    pub fn insert_number(&self, key: &str, value: usize) {        
        self.json.lock().unwrap().as_mut().unwrap().as_object_mut().unwrap().insert(key.to_string(), serde_json::json!(value));

        self.save_data();
    }

    /// Remove a key in the database with its value
    pub fn remove(&self, key: &str) {
        self.json.lock().unwrap().as_mut().unwrap().as_object_mut().unwrap().remove(key);
        
        self.save_data();
    }

    /// Private function but is very important. 
    /// This truncates the db before we write the json code again
    /// And saves the data to the file
    fn save_data(&self) {
        self.file.lock().unwrap().as_ref().unwrap().set_len(0).unwrap();
        self.file.lock().unwrap().as_ref().unwrap().seek(SeekFrom::Start(0)).unwrap();

        self.file.lock().unwrap().as_mut().unwrap().write(serde_json::to_string_pretty(&self.json.lock().unwrap().as_ref().unwrap()).unwrap().as_bytes()).expect("Cannot write to the database!");
    }

    /// Find a value in the db
    pub fn find(&self, key: &str) -> Result<Value, String> {
        let json = &self.json.lock().unwrap();
        let val = &json.as_ref().unwrap()[key];

        if val == &serde_json::Value::Null {
            return Err(format!("The key `{}` does not exist in the database. You might want to create this or handle the error!", key))
        }

        return Ok(Value::from(serde_json::to_string_pretty(&val).unwrap().as_str()));
    }

    /// Check if the key exists in the database
    pub fn contains_key(&self, key: &str) -> bool {
        return self.json.lock().unwrap().as_mut().unwrap().as_object_mut().unwrap().contains_key(key);
    }

    /// Return the length of items that are in the main tree
    pub fn len(&self) -> usize {
        return self.json.lock().unwrap().as_mut().unwrap().as_object_mut().unwrap().len();
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

#[derive(Debug)]
pub struct Tree {
    children: Option<serde_json::Value>
}

impl Tree {
    /// Create a new sub tree
    pub fn new() -> Tree {
        return Tree {
            children: serde_json::from_str("{}").unwrap()
        }
    }

    /// Create a new Tree from String value
    pub fn from(value: &str) -> Tree {
        return Tree {
            children: serde_json::from_str(value).unwrap()
        }
    }

    /// Insert data with [String] value type in the sub tree
    pub fn insert(&mut self, key: &str, value: &str) {
        self.children.as_mut().unwrap().as_object_mut().unwrap().insert(key.to_string(), serde_json::Value::String(value.to_string()));
    }

    /// Insert data with [usize] value type in the sub tree
    pub fn insert_number(&mut self, key: &str, value: usize) {
        self.children.as_mut().unwrap().as_object_mut().unwrap().insert(key.to_string(), serde_json::json!(value));
    }

    /// Return the length of items that are in the sub tree
    pub fn len(&mut self) -> usize {
        return self.children.as_mut().unwrap().as_object_mut().unwrap().len();
    }

    /// Remove a key in the sub tree in the database with its value
    pub fn remove(&mut self, key: &str) {
        self.children.as_mut().unwrap().as_object_mut().unwrap().remove(key);
    }

    /// Find a value in the sub tree in the database
    pub fn find(&self, key: &str) -> Result<Value, String> {
        let val = &self.children.as_ref().unwrap()[key];

        if val == &serde_json::Value::Null {
            return Err(format!("The key `{}` does not exist in the database. You might want to create this or handle the error!", key))
        }

        return Ok(Value::from(serde_json::to_string_pretty(val).unwrap().as_str()));
    }

    /// Check if the key exists in the sub tree of the main database
    pub fn contains_key(&mut self, key: &str) -> bool {
        return self.children.as_mut().unwrap().as_object_mut().unwrap().contains_key(key);
    }

    /// Insert a key with a subtree in the subtree!
    pub fn insert_tree(&mut self, key: &str, value: Tree) {
        self.children.as_mut().unwrap().as_object_mut().unwrap().insert(key.to_string(), serde_json::from_str(value.children.unwrap().to_string().as_str()).unwrap());
    }
}

/// This struct is returned when you find something in the database.
/// Value also impls fmt::Display
pub struct Value {
    val: serde_json::Value
}

/// Impl for Value struct
impl Value {
    /// Value from &str
    fn from(value: &str) -> Value{
        return Value {
            val: serde_json::from_str(value).unwrap()
        }
    }

    /// Return the string value
    pub fn to_string(&self) -> String {
        return serde_json::to_string(&self.val).unwrap();
    }

    /// Return the number value
    pub fn to_number(&self) -> usize {
        return serde_json::to_string(&self.val).unwrap().parse::<usize>().unwrap();
    }

    /// Return the Tree value
    pub fn to_tree(&self) -> Tree {
        return Tree::from(serde_json::to_string(&self.val).unwrap().as_str());
    }
}

/// impl Display for Value
/// So we can print the Value to the screen
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.val {
            serde_json::Value::Null => {
                write!(f, "")
            },

            serde_json::Value::Bool(ref v) => {
                write!(f, "{}", v)
            },

            serde_json::Value::String(ref v) => {
                write!(f, "{}", v)
            },

            serde_json::Value::Number(ref v) => {
                write!(f, "{}", v)
            },

            serde_json::Value::Array(ref v) => {
                write!(f, "{}", serde_json::to_string_pretty(v).unwrap())
            },

            serde_json::Value::Object(ref v) => {
                write!(f, "{}", serde_json::to_string_pretty(v).unwrap())
            },
        }
    }
}

/// impl Display for Tree
/// So we can print the tree to the display
impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", serde_json::to_string_pretty(&self.children).unwrap());
    }
}

/// impl Display for database
/// So we can print the whole tree to the display
impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", serde_json::to_string_pretty(&self.json).unwrap());
    }
}