//imports to use for json file handling and user input
//use std::env;
//use std::io;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

//to go to/from json file
#[derive(Serialize, Deserialize)]
struct UserDB {
    users: HashMap<String, String>,
}

impl UserDB {
    //checks if json file exists, if not creates one
    fn load() -> Self {
        let path = Path::new("users.json");

        if !path.exists() {
            return UserDB { users: HashMap::new() };
        }

        let mut file = File::open(path).expect("Failed to open users.json");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        serde_json::from_str(&contents).unwrap_or(UserDB { users: HashMap::new() })
    }

    //saves to json file
    fn save(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("users.json")
            .unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    //registers new user if username doesn't exist
    fn register(&mut self, username: &str, password: &str) -> bool {
        if self.users.contains_key(username) {
            return false; // username exists
        }
        self.users.insert(username.to_string(), password.to_string());
        self.save();
        true
    }

    //returns true if login is successful, meaing username and password match
    fn login(&self, username: &str, password: &str) -> bool {
        match self.users.get(username) {
            Some(stored) => stored == password,
            None => false,
        }
    }
}

fn main(){

let mut db = UserDB::load();

println!("Welcome to Budget Manager!");
println!("Are you a new user? (y/n)");
let mut y_or_n = String::new();
io::stdin().read_line(&mut y_or_n).expect("Failed to read line");

if y_or_n.trim() == "n" {

println!("Enter username:");
let mut username = String::new();
io::stdin().read_line(&mut username).expect("Failed to read line");
let username = username.trim().to_string();
println!();

println!("Enter password: ");
let mut password = String::new();
io::stdin().read_line(&mut password).expect("Failed to read line");
let password = password.trim().to_string();
println!();

if db.login(&username, &password) {
    println!("\nWelcome back, {}!", username);

    println!("What would you like to do?");
    println!("1. View net worth");
    println!("2. View Budget");
    println!("3. Add Income");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    // Convert input to integer
    let _num: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    match input.trim() {
        "1" => println!("You chose to view your budget."),
        "2" => println!("You chose to add an expense."),
        "3" => println!("You chose to add income."),
        _ => println!("Invalid option."),
    }
} else {
    println!("\nInvalid username or password.");
}

} else if y_or_n.trim() == "y" {
        
        println!("Create a new user account.");
        println!("Enter a username:");

        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Failed");
        let username = username.trim().to_string();

        if db.users.contains_key(&username) {
            println!("Username already exists. Try again.");
            return;
        }

        println!("Enter a password:");
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Failed");
        let password = password.trim().to_string();

        if db.register(&username, &password) {
            println!("\nAccount created! Welcome, {}!", username);
        } else {
            println!("Error creating account.");
        }
    }
}