//imports to use for json file handling and user input
//use std::env;
//use std::io;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
mod options;

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
            return UserDB {
                users: HashMap::new(),
            };
        }

        let mut file = File::open(path).expect("Failed to open users.json");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        serde_json::from_str(&contents).unwrap_or(UserDB {
            users: HashMap::new(),
        })
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
        self.users
            .insert(username.to_string(), password.to_string());
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

fn main() {
    let mut db = UserDB::load();

    println!("Welcome to Budget Manager!"); //Asks if user is new or returning
    println!("Are you a new user? Type (y/n)");
    let mut y_or_n = String::new();
    io::stdin().read_line(&mut y_or_n).expect("Failed to read line");

    if y_or_n.trim() == "n" {
        println!("Enter username:"); //Gets username from user and trims whitespace
        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Failed to read line");
        let username = username.trim().to_string();
        println!();

        println!("Enter password: "); //Gets password from user and trims whitespace
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Failed to read line");
        let password = password.trim().to_string();
        println!();

        //After logging in choose what to do
        if db.login(&username, &password) {
            println!("\nWelcome back, {}!", username);

            loop {
                // Menu options
                println!("What would you like to do? Choose a number:");
                println!("1. View Balances");
                println!("2. View total (before & after debt)");
                println!("3. Add account and total");
                println!("4. Add money to account");
                println!("5. Withdraw money from account");
                println!("6. Remove money account");
                println!("7. Transfer money between your accounts");
                println!("8. Send money");
                println!("9. Logout");
                println!("10. Delete account");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read");

                // Convert input to integer
                let _num: u32 = match input.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Please enter a valid number!");
                        continue;
                    }
                };
                // Match user input to corresponding action
                match input.trim() {
                    "1" => {
                        println!("You chose to view balances.");
                        crate::options::options::view_balances();
                    }
                    "2" => {
                        println!("You chose to view total (before & after debt).");
                        crate::options::options::view_totals();
                    }
                    "3" => {
                        println!("You chose to add account and total.");
                        crate::options::options::add_account_and_balance();
                    }
                    "4" => {
                        println!("You chose to add money to account.");
                        crate::options::options::deposit();
                    }
                    "5" => {
                        println!("You chose to withdraw money from account.");
                        crate::options::options::withdraw();
                    }
                    "6" => {
                        println!("You chose to remove money account.");
                        crate::options::options::remove_account();
                    }
                    "7" => {
                        println!("You chose to transfer money between your accounts.");
                        crate::options::options::internal_transfer();
                    }
                    "8" => {
                        println!("You chose to send money.");
                        crate::options::options::send_money();
                    }
                    "9" => {
                        println!("You chose to logout. Goodbye!");
                        break;
                    }
                    "10" => {
                        println!("You chose to delete account.");
                        crate::options::options::delete_account();
                    }
                    _ => println!("Invalid option."),
                }
            }
        } else {
            println!("\nInvalid username or password.");
        }
    } else if y_or_n.trim() == "y" {
        //Gets new username and password from user
        println!("Create a new user account.");
        println!("Enter a username:");

        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Failed");
        let username = username.trim().to_string();

        if db.users.contains_key(&username) {
            //checks if username already exists
            println!("Username already exists. Try again.");
            return;
        }

        println!("Enter a password:");
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Failed");
        let password = password.trim().to_string();

        if db.register(&username, &password) {
            println!("\nAccount created! Welcome, {}!", username);

            loop {
                // Menu options
                println!("What would you like to do? Choose a number:");
                println!("1. View Balances");
                println!("2. View total (before & after debt)");
                println!("3. Add account and total");
                println!("4. Add money to account");
                println!("5. Withdraw money from account");
                println!("6. Remove money account");
                println!("7. Transfer money between your accounts");
                println!("8. Send money");
                println!("9. Logout");
                println!("10. Delete account");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read");

                // Convert input to integer
                let _num: u32 = match input.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Please enter a valid number!");
                        continue;
                    }
                };
                // Match user input to corresponding action
                match input.trim() {
                    "1" => {
                        println!("You chose to view balances.");
                        crate::options::options::view_balances();
                    }
                    "2" => {
                        println!("You chose to view total (before & after debt).");
                        crate::options::options::view_totals();
                    }
                    "3" => {
                        println!("You chose to add account and total.");
                        crate::options::options::add_account_and_balance();
                    }
                    "4" => {
                        println!("You chose to add money to account.");
                        crate::options::options::deposit();
                    }
                    "5" => {
                        println!("You chose to withdraw money from account.");
                        crate::options::options::withdraw();
                    }
                    "6" => {
                        println!("You chose to remove money account.");
                        crate::options::options::remove_account();
                    }
                    "7" => {
                        println!("You chose to transfer money between your accounts.");
                        crate::options::options::internal_transfer();
                    }
                    "8" => {
                        println!("You chose to send money.");
                        crate::options::options::send_money();
                    }
                    "9" => {
                        println!("You chose to logout. Goodbye!");
                        break;
                    }
                    "10" => {
                        println!("You chose to delete account.");
                        crate::options::options::delete_account();
                    }
                    _ => println!("Invalid option."),
                }
            }
        } else {
            println!("Error creating account.");
        }
    }
}