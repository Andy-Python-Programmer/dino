use std::fs::{ OpenOptions, File };
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
            .append(false)
            .truncate(true)
            .open(&self.path.to_string())
            .unwrap();
        
        let mut buf = String::new();

        file.read_to_string(&mut buf).unwrap();

        self.file = Some(file);
        self.data = Some(buf);
        self.json = Some(serde_json::from_str("{}").unwrap());
    }

    /// Insert a key with a subtree in the database
    pub fn insert_tree(&mut self, key: &str, value: Tree) {
        self.file.as_ref().unwrap().set_len(0).unwrap();
        self.json.as_mut().unwrap().as_object_mut().unwrap().insert(key.to_string(), serde_json::from_str(value.children.unwrap().to_string().as_str()).unwrap());
        self.file.as_mut().unwrap().write(self.json.as_ref().unwrap().to_string().as_bytes()).expect("Cannot write to the database!");
    }

    /// Insert a key and a value in the database
    pub fn insert(&mut self, key: &str, value: &str) {
        self.file.as_ref().unwrap().set_len(0).unwrap();
        self.json.as_mut().unwrap().as_object_mut().unwrap().insert(key.to_string(), serde_json::json!(value));
        self.file.as_mut().unwrap().write(self.json.as_ref().unwrap().to_string().as_bytes()).expect("Cannot write to the database!");
    }

    /// Find a value in the db
    pub fn find(&self, key: &str) -> &serde_json::Value {
        let val = &self.json.as_ref().unwrap()[key];

        return val;
    }
}

pub struct Tree {
    pub children: Option<serde_json::Value>
}

impl Tree {
    pub fn new() -> Tree {
        return Tree {
            children: serde_json::from_str("{}").unwrap()
        }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.children.as_mut().unwrap().as_object_mut().unwrap().insert(key.to_string(), serde_json::Value::String(value.to_string()));
    }
}